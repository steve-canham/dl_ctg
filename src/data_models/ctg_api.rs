
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct CTGRootobject
{
    pub studies: Option<Vec<CTGStudy>>,
    #[serde(rename = "nextPageTokenSection")]
    pub next_page_token: Option<String>,
    #[serde(rename = "totalCount")]
    pub total_count: Option<i32>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct CTGStudy
{
    #[serde(rename = "protocolSection")]
    pub protocol_section: ProtocolSection,
    #[serde(rename = "derivedSection")]
    pub derived_section: Option<DerivedSection>,
    #[serde(rename = "documentSection")]
    pub document_section: Option<DocumentSection>,
    #[serde(rename = "hasResults")]
    pub has_results: Option<bool>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ProtocolSection
{
    #[serde(rename = "identificationModule")]
    pub identification_module: IdentificationModule,
    #[serde(rename = "statusModule")]
    pub status_module: StatusModule,
    #[serde(rename = "sponsorCollaboratorsModule")]
    pub sponsor_collaborators_module: Option<SponsorCollaboratorsModule>,
    #[serde(rename = "descriptionModule")]
    pub description_module: Option<DescriptionModule>,
    #[serde(rename = "conditionsModule")]
    pub conditions_module: Option<ConditionsModule>,
    #[serde(rename = "designModule")]
    pub design_module: Option<DesignModule>,
    #[serde(rename = "eligibilityModule")]
    pub eligibility_module: Option<EligibilityModule>,
    #[serde(rename = "contactsLocationsModule")]
    pub contacts_locations_module: Option<ContactsLocationsModule>,    
    #[serde(rename = "referencesModule")]
    pub references_module: Option<ReferencesModule>,
    #[serde(rename = "ipdSharingStatementModule")]
    pub ipd_sharing_statement_module: Option<IPDSharingStatementModule>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct IdentificationModule
{
    #[serde(rename = "nctId")]
    pub nct_id: String,
    #[serde(rename = "nctIdAliases")]
    pub nct_id_aliases: Option<Vec<String>>,   
    #[serde(rename = "orgStudyIdInfo")]
    pub org_study_id_info: Option<OrgStudyIdInfo>, 
    #[serde(rename = "secondaryIdInfos")]
    pub secondary_id_infos: Option<Vec<SecondaryIdInfos>>, 
    #[serde(rename = "briefTitle")]
    pub brief_title: Option<String>,
    #[serde(rename = "officialTitle")]
    pub official_title: Option<String>,
    pub acronym: Option<String>,
    pub organization: Option<Organization>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct OrgStudyIdInfo
{
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub id_type: Option<String>,
    pub link: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct SecondaryIdInfos
{
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub id_type: Option<String>,
    pub domain: Option<String>,    
    pub link: Option<String>,
}


#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Organization
{
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    #[serde(rename = "class")]
    pub org_class: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct StatusModule
{
    #[serde(rename = "statusVerifiedDate")]
    pub status_verified_date: Option<String>,
    #[serde(rename = "overallStatus")]
    pub overall_status: Option<String>,
    #[serde(rename = "lastKnownStatus")]
    pub last_known_status: Option<String>,
    #[serde(rename = "whyStopped")]
    pub why_stopped: Option<String>,
    #[serde(rename = "expandedAccessInfo")]
    pub expanded_access_info: Option<ExpandedAccessInfo>,

    #[serde(rename = "startDateStruct")]
    pub start_date: Option<DateStruct>,
    #[serde(rename = "primaryCompletionDateStruct")]
    pub primary_comp_date: Option<DateStruct>,
    #[serde(rename = "completionDateStruct")]
    pub comp_date: Option<DateStruct>,
    #[serde(rename = "studyFirstPostDateStruct")]
    pub study_posted_date: Option<DateStruct>,
    #[serde(rename = "resultsFirstPostDateStruct")]
    pub results_posted_date: Option<DateStruct>,
    #[serde(rename = "lastUpdatePostDateStruct")]
    pub last_updated_date: Option<DateStruct>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ExpandedAccessInfo
{
    #[serde(rename = "hasExpandedAccess")]
    pub has_expanded_access: Option<bool>,
    #[serde(rename = "nctId")]
    pub nct_id: Option<String>,
    #[serde(rename = "statusForNctId")]
    pub status_for_nct_id: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct DateStruct
{
    pub date: Option<String>,
    #[serde(rename = "type")]
    pub date_type: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct SponsorCollaboratorsModule
{
    #[serde(rename = "responsibleParty")]
    pub responsible_party: Option<ResponsibleParty>,
    #[serde(rename = "leadSponsor")]    
    pub lead_sponsor: Option<Sponsor>,
    pub collaborators: Option<Vec<Sponsor>>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Sponsor
{
    pub name: Option<String>,
    #[serde(rename = "class")]
    pub sponsor_class: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ResponsibleParty
{
    #[serde(rename = "type")]
    pub rp_type: Option<String>,
    #[serde(rename = "investigatorFullName")]
    pub investigator_full_name: Option<String>,
    #[serde(rename = "investigatorTitle")]
    pub investigator_title: Option<String>,
    #[serde(rename = "investigatorAffiliation")]
    pub investigator_affiliation: Option<String>,
    #[serde(rename = "oldNameTitle")]
    pub old_name_title: Option<String>,
    #[serde(rename = "oldOrganization")]
    pub old_organization: Option<String>,
}


#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct DescriptionModule
{
    #[serde(rename = "briefSummary")]
    pub brief_summary: Option<String>,
    #[serde(rename = "detailedDescription")]
    pub detailed_description: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ConditionsModule
{
    pub conditions: Option<Vec<String>>, 
    pub keywords: Option<Vec<String>>, 
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct DesignModule
{
    #[serde(rename = "studyType")]
    pub study_type: Option<String>,
    #[serde(rename = "patientRegistry")]
    pub patient_registry: Option<bool>, 
    pub phases: Option<Vec<String>>,  
    #[serde(rename = "designInfo")]
    pub design_info: Option<DesignInfo>,
    #[serde(rename = "enrollmentInfo")]
    pub enrollment_info: Option<EnrollmentInfo>,
    #[serde(rename = "bioSpec")]
    pub bio_spec: Option<Biospec>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct DesignInfo
{
    pub allocation: Option<String>,
    #[serde(rename = "interventionModel")]
    pub intervention_model: Option<String>,
    #[serde(rename = "interventionModelDescription")]
    pub intervention_model_description: Option<String>, 
    #[serde(rename = "primaryPurpose")]
    pub primary_purpose: Option<String>,
    #[serde(rename = "observationalModel")]
    pub observational_model: Option<String>,
    #[serde(rename = "timePerspective")]
    pub time_perspective: Option<String>,
    #[serde(rename = "maskingInfo")]
    pub masking_info: Option<MaskingInfo>
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct MaskingInfo
{
    pub masking: Option<String>,
    #[serde(rename = "maskingDescription")]
    pub masking_description: Option<String>,
    #[serde(rename = "whoMasked")]
    pub who_masked: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct EnrollmentInfo
{
    pub count: Option<i32>,
    #[serde(rename = "type")]
    pub enrol_type: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Biospec
{
    pub retention: Option<String>,
    pub description: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct EligibilityModule
{
    #[serde(rename = "eligibilityCriteria")]
    pub eligibility_criteria: Option<String>,
    #[serde(rename = "healthyVolunteers")]
    pub healthy_volunteers: Option<bool>,
    pub sex: Option<String>,
    #[serde(rename = "genderBased")]
    pub gender_based: Option<bool>,
    #[serde(rename = "genderDescription")]
    pub gender_description: Option<String>,
    #[serde(rename = "minimumAge")]
    pub minimum_age: Option<String>,
    #[serde(rename = "maximumAge")]
    pub maximum_age: Option<String>,
    #[serde(rename = "stdAges")]
    pub std_ages: Option<Vec<String>>,
    #[serde(rename = "studyPopulation")]
    pub study_population: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ContactsLocationsModule
{
    #[serde(rename = "centralContacts")]
    pub central_contacts: Option<Vec<CentralContact>>,
    #[serde(rename = "overallOfficials")]
    pub overall_officials: Option<Vec<OverallOfficial>>,
    pub locations: Option<Vec<Location>>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct CentralContact
{
    pub name: Option<String>,
    pub role: Option<String>,
    pub email: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct OverallOfficial
{
    pub name: Option<String>,
    pub affiliation: Option<String>,
    pub role: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Location
{
    pub country: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ReferencesModule
{
    pub references: Option<Vec<Reference>>,
    #[serde(rename = "seeAlsoLinks")]
    pub see_also_links: Option<Vec<SeeAlsoLink>>,
    #[serde(rename = "availIpds")]
    pub avail_ipds: Option<Vec<AvailIpd>>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Reference
{
    pub pmid: Option<String>,
    #[serde(rename = "type")]
    pub ref_type: Option<String>,
    pub citation: Option<String>,
    pub retractions: Option<Vec<Retraction>>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Retraction
{
    pub pmid: Option<String>,
    pub source: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct SeeAlsoLink
{
    pub label: Option<String>,
    pub url: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct AvailIpd
{
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub ipd_type: Option<String>,
    pub url: Option<String>,
    pub comment: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct IPDSharingStatementModule
{
    #[serde(rename = "ipdSharing")]
    pub ipd_sharing: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "infoTypes")]
    pub info_types: Option<Vec<String>>,
    #[serde(rename = "timeFrame")]
    pub time_frame: Option<String>,
    #[serde(rename = "accessCriteria")]
    pub access_criteria: Option<String>,
    pub url: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct DocumentSection
{
    #[serde(rename = "largeDocumentModule")]
    pub large_document_module: Option<LargeDocumentModule>, 
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct LargeDocumentModule
{
    #[serde(rename = "largeDocs")]
    pub large_docs: Option<Vec<LargeDoc>>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct LargeDoc
{
    #[serde(rename = "typeAbbrev")]
    pub type_abbrev: Option<String>,
    #[serde(rename = "hasProtocol")]
    pub has_protocol: Option<bool>,
    #[serde(rename = "hasSap")]
    pub has_sap: Option<bool>,
    #[serde(rename = "hasIcf")]
    pub has_icf: Option<bool>,
    pub label: Option<String>,
    pub date: Option<String>,
    #[serde(rename = "uploadDate")]
    pub upload_date: Option<String>,
    pub filename: Option<String>,
    pub size: Option<i32>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct DerivedSection
{
    #[serde(rename = "conditionBrowseModule")]
    pub condition_browse_module: Option<ConditionBrowseModule>,
    #[serde(rename = "interventionBrowseModule")]
    pub intervention_browse_module: Option<InterventionBrowseModule>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ConditionBrowseModule
{
    pub meshes: Option<Vec<Mesh>>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct InterventionBrowseModule
{
    pub meshes: Option<Vec<Mesh>>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Mesh
{
    pub id: Option<String>,
    pub term: Option<String>,
}
    
