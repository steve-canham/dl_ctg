use crate::data_models::ctg_api::{CTGStudy, DateStruct};
use crate::data_models::json_models::*;
use crate::err::AppError;
use chrono::{Utc, SecondsFormat};


pub fn process_study (study: CTGStudy) -> Result<Study, AppError> {
    
    // Protocol section, identification module, status module,
    // NCT ID and date first posted are all expected to exist for every file.
    // Examination of full DB dump files indicate that this is the case.

    let sp = study.protocol_section;
    let idm = sp.identification_module; 
    let sm = sp.status_module; 

    let sd_sid = idm.nct_id;
    let public_title = idm.brief_title;
    let scientific_title = idm.official_title;
    let acronym = idm.acronym;

    let mut identifiers: Vec<Identifier> = Vec::new();

    identifiers.push ( Identifier { 
                source: "primary".to_string(),
                value: sd_sid.clone(), 
                id_type:  Some("nct id".to_string()), 
                org: Some("national Library of medicine".to_string()),
                link: None });

    if let Some(aliases) = idm.nct_id_aliases {
        for a in aliases {
            identifiers.push ( Identifier { 
                source: "alias".to_string(),
                value: a, 
                id_type: Some("obsolete NCT number".to_string()), 
                org: Some("national Library of medicine".to_string()),
                link: None });
            }
    }

    let mut prim_org_name = None;
    if let Some(org) = idm.organization {
        prim_org_name = org.full_name;
    }

    if let Some(gs_id) = idm.org_study_id_info {
        if let Some(id) = gs_id.id {
            identifiers.push ( Identifier { 
                source: "sponsor".to_string(),
                value: id, 
                id_type:  gs_id.id_type, 
                org: prim_org_name,
                link: gs_id.link });
        }
    }

    if let Some(sec_ids) = idm.secondary_id_infos {
        for sec_id in sec_ids {
            if let Some(id) = sec_id.id {
                identifiers.push ( Identifier { 
                    source: "secondary".to_string(),
                    value: id, 
                    id_type:  sec_id.id_type, 
                    org: sec_id.domain,
                    link: sec_id.link });
            }
        }
    }

    let (date_first_posted, date_first_posted_type) = process_date_struct(&sm.study_posted_date);
    let (date_last_updated, date_last_updated_type) = process_date_struct(&sm.last_updated_date);
    let registration_dates = Registration {
        date_first_posted: date_first_posted.clone(),
        date_first_posted_type,
        date_last_updated,
        date_last_updated_type,
    };


    let (study_start, study_start_type) = process_date_struct(&sm.start_date);
    let (primary_comp, primary_comp_type) = process_date_struct(&sm.primary_comp_date);
    let (completion, completion_type) = process_date_struct(&sm.comp_date);
    let (results_posted, results_posted_type) = process_date_struct(&sm.results_posted_date);
    let study_dates = StudyDates {
        study_start,
        study_start_type,
        primary_comp,
        primary_comp_type,
        completion,
        completion_type,
        results_posted,
        results_posted_type,
    };


    let mut has_expanded_access = None;
    let mut ea_nct_id = None;
    let mut status_for_nct_id = None;
    if let Some(eai) = sm.expanded_access_info {
        has_expanded_access = eai.has_expanded_access;
        ea_nct_id = eai.nct_id;
        status_for_nct_id = eai.status_for_nct_id;
    }
    let study_status = Status {
        overall_status: sm.overall_status,
        last_known_status: sm.last_known_status,
        has_results: study.has_results,
        status_verified_date: sm.status_verified_date,
        why_stopped: sm.why_stopped,
        has_expanded_access: has_expanded_access,
        ea_nct_id: ea_nct_id,
        status_for_nct_id: status_for_nct_id,
    };


    let mut orgs: Vec<Organisation> = Vec::new();
    let mut people: Vec<Person> = Vec::new();
  
    if let Some (scm) = sp.sponsor_collaborators_module {

        if let Some(rp) = scm.responsible_party {

            let mut inv_name = rp.investigator_full_name;
            if let Some(old_n) = rp.old_name_title {
                inv_name = Some(old_n);
            }

            let mut inv_affil = rp.investigator_affiliation.clone();
            if let Some(old_g) = rp.old_organization {
                if rp.investigator_affiliation == None {
                    inv_affil = Some(old_g);
                }
            }

            if inv_name.is_some() {
                people.push (Person {
                    source: "resp_party".to_string(),
                    role: rp.rp_type,
                    title: rp.investigator_title,
                    name: inv_name,
                    affiliation: inv_affil,
                    email: None
                });
            }
        }

        if let Some(sp) = scm.lead_sponsor {
            if let Some(nm) = sp.name {
                orgs.push ( Organisation { 
                    org_type: "sponsor".to_string(), 
                    org_name: nm, 
                    org_class: sp.sponsor_class }
                );
            }
        }

        if let Some(cos) = scm.collaborators {
            for co in cos {
                if let Some(nm) = co.name {
                    orgs.push ( Organisation { 
                        org_type: "collaborator".to_string(), 
                        org_name: nm, 
                        org_class: co.sponsor_class }
                    );
                }
            }
        }

    }


    let mut brief_desc = None;
    let mut detailed_desc = None;
    if let Some (dm) = sp.description_module {
        brief_desc = dm.brief_summary;
        detailed_desc = dm.detailed_description;
    }
 
    let mut design = None;
    let mut enrolment = None;

    if let Some (dsm) = sp.design_module {

        if let Some(eninf) = dsm.enrollment_info {
            let mut enrol_type = None;
            if let Some(en_type) = eninf.enrol_type {
                enrol_type = match en_type.as_str() {
                    "ACTUAL" => Some("a".to_string()),
                    "ESTIMATED" => Some("e".to_string()),
                    _ => None,
                };
            }
            enrolment = Some(Enrolment {
                enrol_numbers: eninf.count,
                enrol_type: enrol_type,
            });
        }

        let mut allocation = None;
        let mut intervention_model = None; 
        let mut intervention_model_description = None;
        let mut primary_purpose = None;

        let mut phases = None;
        let mut masking = None;
        let mut masking_description = None;
        let mut who_masked = None;

        let mut observational_model = None;
        let mut time_perspective = None;
        let mut bio_spec_retention = None;
        let mut bio_spec_description = None;

        if let Some(phs) = dsm.phases {
            phases = Some(phs.join(","));
        }

        if let Some(di) = dsm.design_info {

            allocation = di.allocation;
            intervention_model = di.intervention_model; 
            intervention_model_description = di.intervention_model_description;
            primary_purpose = di.primary_purpose;
 
            if let Some(mi) = di.masking_info {
                masking = mi.masking;
                masking_description = mi.masking_description;
                if let Some(wms) = mi.who_masked {
                    who_masked = Some(wms.join(","));
                }
            }

            observational_model = di.observational_model;
            time_perspective = di.time_perspective;
        }

        if let Some(bs) = dsm.bio_spec {
            bio_spec_retention = bs.retention;
            bio_spec_description = bs.description;
        }
  
       design = Some(Design {
            study_type: dsm.study_type,
            patient_registry: dsm.patient_registry,
            phases: phases,
            allocation: allocation,
            intervention_model: intervention_model, 
            intervention_model_description: intervention_model_description,
            primary_purpose: primary_purpose,
            masking: masking,
            masking_description: masking_description,
            who_masked: who_masked,
            observational_model: observational_model,
            time_perspective: time_perspective,
            bio_spec_retention: bio_spec_retention,
            bio_spec_description: bio_spec_description,
       });

    }


    let mut partics = None;
    let mut std_ages = Vec::new();

    if let Some (em) = sp.eligibility_module 
    {

        partics = Some(Participants {
            eligibility_criteria: em.eligibility_criteria,
            healthy_volunteers: em.healthy_volunteers,
            sex: em.sex,
            gender_based: em.gender_based,
            gender_description: em.gender_description,
            minimum_age: em.minimum_age,
            maximum_age: em.maximum_age,
            study_population: em.study_population,         
        });
        
        if let Some(ages) = em.std_ages {
            for age in ages {
                std_ages.push(age);
            }
        }
    }

    let mut countries: Vec<String> = Vec::new();

    if let Some (clm) = sp.contacts_locations_module {

        if let Some(ccs) = clm.central_contacts {
            for cc in ccs {
                people.push (Person {
                    source: "central_contact".to_string(),
                    role: cc.role,
                    title: None,
                    name: cc.name,
                    affiliation: None,
                    email: cc.email
                });
            }
        }

        if let Some(oos) = clm.overall_officials {
            for oo in oos {
                people.push (Person {
                    source: "overall_official".to_string(),
                    role: oo.role,
                    title: None,
                    name: oo.name,
                    affiliation: oo.affiliation,
                    email: None
                });
            }
        }

        if let Some(locs) = clm.locations {
            for loc in locs {
                if let Some(c)  = loc.country {

                    // Check country not already listed

                    let mut add = true;
                    for ex_c in &countries {
                        if *ex_c == c {
                            add = false;
                            break;
                        }
                    }
                    if add {
                        countries.push(c);  
                    }
                }
            }
        }

    }


    let mut references: Vec<Reference> = Vec::new();
    let mut avail_ipds: Vec<AvailIpd> = Vec::new();
    let mut links: Vec<Link> = Vec::new(); 

    if let Some (rm) = sp.references_module {

        if let Some(refs) = rm.references {
            for rf in refs  {
                if rf.ref_type != Some("BACKGROUND".to_string()) {
                    let mut has_retraction = false;
                    let mut retractions = "".to_string();
                    if let Some(rets) = rf.retractions {
                        has_retraction = true;
                        for ret in rets {
                            let ret_string = format!("{}: {}", 
                                    ret.pmid.unwrap_or_default(), 
                                    ret.source.unwrap_or_default());
                            if retractions == "".to_string() {
                                retractions = ret_string;
                            } else {
                                retractions = format!("{}, {}", retractions, ret_string);
                            }
                        }
                    }

                    references.push(Reference {
                        pmid: rf.pmid,
                        ref_type: rf.ref_type,
                        citation: rf.citation,
                        retracted: has_retraction,
                        retractions: Some(retractions),
                    });
                }
            }
        }

        if let Some(ipd_docs) = rm.avail_ipds {
            for ipd_doc in ipd_docs  {
                avail_ipds.push(AvailIpd { 
                    id: ipd_doc.id, 
                    ipd_type: ipd_doc.ipd_type, 
                    url: ipd_doc.url, 
                    comment: ipd_doc.comment
                });
            }
        }


        if let Some(lnks) = rm.see_also_links {
            for lnk in lnks  {
                links.push(Link { 
                    label: lnk.label,
                    url: lnk.url }
                );
            }
        }


    }

    let mut ipd_data = None;
    if let Some (ipd) = sp.ipd_sharing_statement_module {
  
        let mut info_types = None;
        if let Some(ilist) = ipd.info_types {
            info_types = Some(ilist.join(","));
        }
        
        ipd_data = Some(IPD {
            ipd_sharing: ipd.ipd_sharing,   
            description: ipd.description,
            info_types: info_types,
            time_frame: ipd.time_frame,
            access_criteria: ipd.access_criteria,
            url: ipd.url,
        });
    }


    let mut documents: Vec<StudyDoc> = Vec::new();

    if let Some(doc_sec) = study.document_section {
        if let Some(ldm) = doc_sec.large_document_module {
            if let Some(docs) = ldm.large_docs {
                for d in docs {
                    documents.push(StudyDoc {
                        type_abbrev: d.type_abbrev,
                        has_protocol: d.has_protocol,
                        has_sap: d.has_sap,
                        has_icf: d.has_icf,
                        label: d.label,
                        date: d.date,
                        upload_date: d.upload_date,
                        filename: d.filename,
                        size: d.size,
                    });
                }
            }
        }
    }

    let mut conditions: Vec<Condition> = Vec::new();
    let mut interventions: Vec<Intervention> = Vec::new();
    let mut keywords: Vec<String> = Vec::new();

    if let Some(der_sec) = study.derived_section {

        if let Some(cbm) = der_sec.condition_browse_module {
            if let Some(meshes) = cbm.meshes {
                for m in meshes {
                    if let Some(t) = m.term {
                        conditions.push(Condition {
                            term: t,
                            id: m.id,
                            id_type: Some("MESH".to_string()),
                        });
                    }
                }
            }
        }

        if let Some(ibm) = der_sec.intervention_browse_module {
            if let Some(meshes) = ibm.meshes {
                for m in meshes {
                    if let Some(t) = m.term {
                        interventions.push(Intervention {
                            term: t,
                            id: m.id,
                            id_type: Some("MESH".to_string()),
                        });
                    }
                }
            }
        }
    }


    if let Some (csm) = sp.conditions_module {
        if let Some(cons) = csm.conditions {
            for con in cons {
                // Need to check not already covered by Mesh terms
                if con.trim() != "" {
                    let mut add = true;
                    for c in &conditions {
                        if con == c.term {
                            add = false;
                            break;
                        }
                    }
                    if add {
                        conditions.push(Condition {
                            term: con,
                            id: None,
                            id_type: None,
                        });
                    }
                }
            }
        }

        if let Some(kws) = csm.keywords {
            for kw in kws {
                keywords.push(kw);
            }
        }
    }


    let study = Study {
        sd_sid: sd_sid.clone(), 
        downloaded: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, false),

        public_title: public_title,
        scientific_title: scientific_title,
        acronym: acronym,
        identifiers: array_as_option(identifiers),

        brief_description:  brief_desc,
        detailed_description: detailed_desc,

        registration: registration_dates,
        study_dates: study_dates,
        study_status: study_status,

        organisations: array_as_option(orgs),
        people: array_as_option(people),

        design: design,
        enrolment: enrolment,
        participants: partics,
        age_groups: array_as_option(std_ages),

        conditions: array_as_option(conditions),
        interventions: array_as_option(interventions),
        keywords: array_as_option(keywords),

        countries: array_as_option(countries),

        documents: array_as_option(documents),
        references: array_as_option(references),
        avail_ipd_docs: array_as_option(avail_ipds),
        links: array_as_option(links),
        ipd: ipd_data,
    };


    Ok(study)


}



fn process_date_struct(ds: &Option<DateStruct>) -> (Option<String>, Option<String>) {
    
    if let Some(dsd) = ds {
        let mut dtype: Option<String> = None;
        if let Some(dt) = &dsd.date_type {
            dtype = match dt.as_str(){
                "ACTUAL" => Some("a".to_string()),
                "ESTIMATED" => Some("e".to_string()),
                _ => None,
            };
        }
        (dsd.date.clone(), dtype)
    }
    else {
        (None, None)
    }
}

fn array_as_option<T>(a: Vec<T>) -> Option<Vec<T>> {
    match a.len() {
        0 => None,
        _ => Some(a),
    }
}
