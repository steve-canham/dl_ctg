use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Study
{
    pub sd_sid: String, 
    pub downloaded: String,

    pub public_title: Option<String>,
    pub scientific_title: Option<String>,
    pub acronym: Option<String>,
    
    pub identifiers: Option<Vec<Identifier>>,
        
    pub brief_description: Option<String>,
    pub detailed_description: Option<String>,

    pub registration: Registration,
    pub study_dates: StudyDates,
    pub study_status: Status,

    pub organisations: Option<Vec<Organisation>>,
    pub people: Option<Vec<Person>>,

    pub design: Option<Design>,
    pub enrolment:Option<Enrolment>,
    pub participants: Option<Participants>,
    pub age_groups: Option<Vec<String>>,
    
    pub conditions: Option<Vec<Condition>>,
    pub interventions: Option<Vec<Intervention>>,
    pub keywords: Option<Vec<String>>,
   
    pub countries: Option<Vec<String>>,

    pub documents: Option<Vec<StudyDoc>>,
    pub references: Option<Vec<Reference>>,
    pub avail_ipd_docs: Option<Vec<AvailIpd>>,
    pub links: Option<Vec<Link>>,
    pub ipd: Option<IPD>,

}


#[derive(Serialize, Deserialize)]
pub struct Identifier
{
    pub source: String,
    pub value: String,
    pub id_type: Option<String>,
    pub org: Option<String>,
    pub link: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct Registration
{
    pub date_first_posted: Option<String>,
    pub date_first_posted_type: Option<String>,
    pub date_last_updated: Option<String>,
    pub date_last_updated_type: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct StudyDates
{
    pub study_start: Option<String>,
    pub study_start_type: Option<String>,
    pub primary_comp: Option<String>,
    pub primary_comp_type: Option<String>,
    pub completion: Option<String>,
    pub completion_type: Option<String>,
    pub results_posted: Option<String>,
    pub results_posted_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Status
{
    pub overall_status: Option<String>,
    pub last_known_status: Option<String>,
    pub has_results: Option<bool>,
    pub status_verified_date: Option<String>,
    pub why_stopped: Option<String>,
    pub has_expanded_access: Option<bool>,
    pub ea_nct_id: Option<String>,
    pub status_for_nct_id: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct Organisation
{
    pub org_type: String,
    pub org_name: String,
    pub org_class: Option<String>,
}



#[derive(Serialize, Deserialize)]
pub struct Design
{
    pub study_type: Option<String>,
    pub patient_registry: Option<bool>,

    pub phases: Option<String>,  

    pub allocation: Option<String>,
    pub intervention_model: Option<String>,
    pub intervention_model_description: Option<String>, 
    pub primary_purpose: Option<String>,

    pub masking: Option<String>,
    pub masking_description: Option<String>,
    pub who_masked: Option<String>,

    pub observational_model: Option<String>,
    pub time_perspective: Option<String>,

    pub bio_spec_retention: Option<String>,
    pub bio_spec_description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Enrolment
{
    pub enrol_numbers: Option<i32>,
    pub enrol_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Participants
{
    pub eligibility_criteria: Option<String>,
    pub healthy_volunteers: Option<bool>,
    pub sex: Option<String>,
    pub gender_based: Option<bool>,
    pub gender_description: Option<String>,
    pub minimum_age: Option<String>,
    pub maximum_age: Option<String>,
    pub study_population: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct Condition
{
    pub term: String,
    pub id: Option<String>,
    pub id_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Intervention
{
    pub term: String,
    pub id: Option<String>,
    pub id_type: Option<String>,
}

/* 
#[derive(Serialize, Deserialize)]
pub struct StudyCentre
{
    pub name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
}
*/


#[derive(Serialize, Deserialize)]
pub struct Person
{
    pub source: String,
    pub role: Option<String>,
    pub title: Option<String>,
    pub name: Option<String>,
    pub affiliation: Option<String>,
    pub email: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct RespParty
{
    pub rp_type: Option<String>,
    pub investigator_full_name: Option<String>,
    pub investigator_title: Option<String>,
    pub investigator_affiliation: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct Reference
{
    pub pmid: Option<String>,
    pub ref_type: Option<String>,
    pub citation: Option<String>,
    pub retracted: bool,
    pub retractions: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct AvailIpd
{
    pub id: Option<String>,
    pub ipd_type: Option<String>,
    pub url: Option<String>,
    pub comment: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct Link
{
    pub label: Option<String>,
    pub url: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct IPD
{
    pub ipd_sharing: Option<String>,   
    pub description: Option<String>,
    pub info_types: Option<String>,
    pub time_frame: Option<String>,
    pub access_criteria: Option<String>,
    pub url: Option<String>,
}



#[derive(Serialize, Deserialize)]
pub struct StudyDoc
{
    pub type_abbrev: Option<String>,
    pub has_protocol: Option<bool>,
    pub has_sap: Option<bool>,
    pub has_icf: Option<bool>,
    pub label: Option<String>,
    pub date: Option<String>,
    pub upload_date: Option<String>,
    pub filename: Option<String>,
    pub size: Option<i32>,
}
