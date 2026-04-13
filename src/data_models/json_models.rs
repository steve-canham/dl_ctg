use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Study
{
    pub sd_sid: String, 
    pub downloaded: String,

    pub public_title: Option<String>,
    pub scientific_title: Option<String>,
    pub acronym: Option<String>,
    
    pub registration: Registration,
    pub identifiers: Option<Vec<Identifier>>,
    
    pub summary: Summary,

    pub design: Design,
    pub trial_types: Option<Vec<String>>,
    pub trial_settings: Option<Vec<String>>,

    pub conditions: Option<Vec<Condition>>,
    pub interventions: Option<Vec<Intervention>>,

    pub contacts: Option<Vec<StudyContact>>,
    pub sponsors: Option<Vec<StudySponsor>>,
    pub funders: Option<Vec<StudyFunder>>,

    pub participant_types: Option<Vec<String>>,
    pub participants: Participants,

    pub recruitment: Recruitment,
    pub countries: Option<Vec<String>>,

    pub data_policies: Option<Vec<String>>,
    pub results: Results,
    pub links: Option<Vec<StudyLink>>,
    pub files: Option<Vec<StudyFile>>,
    pub ipd: IPD,

}

#[derive(Serialize, Deserialize)]
pub struct Registration
{
    pub date_id_assigned: Option<String>,
    pub last_updated: Option<String>,
}

// Structs for titles and identifiers have differebt form
// as trhey are only created when the source data exists,
// and therefore do not need to be 'Options',
// and are created most easily using the new() function

#[derive(Serialize, Deserialize)]
pub struct Identifier
{
    pub identifier_type_id: i32,
    pub identifier_type: String,
    pub identifier_value: String,
}

impl Identifier {
    pub fn new(identifier_type_id: i32, identifier_type: String, identifier_value: String) -> Self {
        Identifier {
            identifier_type_id,
            identifier_type,
            identifier_value,
        }
   }
}

#[derive(Serialize, Deserialize)]
pub struct Summary
{
    pub plain_english_summary: Option<String>,
    pub study_hypothesis: Option<String>,
    pub primary_outcome: Option<String>,
    pub secondary_outcome: Option<String>,
    pub overall_end_date: Option<String>,
    pub trial_website: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct Ethics
{
    pub ethics_approval_required: Option<String>,
    pub ethics_approval: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EthicsCommittee
{
    pub name: Option<String>,
    pub approval_status: Option<String>,
    pub status_date: Option<String>,
    pub committee_reference: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OutcomeMeasure
{
    pub variable: Option<String>,
    pub method: Option<String>,
    pub timepoints: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Design
{
    pub study_design: Option<String>,
    pub primary_study_design: Option<String>,
    pub secondary_study_design: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Condition
{
    pub description: Option<String>,
    pub disease_class1: Option<String>,
    pub disease_class2: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct Intervention 
{
    pub description: Option<String>,
    pub int_type: Option<String>,
    pub pharma_study_types: Option<String>,
    pub phase: Option<String>,
    pub drug_names: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct StudyContact
{
    pub title: Option<String>,
    pub forename: Option<String>,
    pub surname: Option<String>,
    pub orcid: Option<String>,
    pub contact_types: Option<Vec<String>>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub privacy: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StudySponsor
{
    pub organisation: Option<String>,
    pub website: Option<String>,
    pub sponsor_type: Option<String>,
    pub ror_id: Option<String>,  
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub privacy: Option<String>,
    pub commercial_status: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StudyFunder
{
    pub name: Option<String>,
    pub fund_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Participants
{
    pub age_range: Option<String>,
    pub l_age_limit: Option<String>,
    pub l_age_limit_num: Option<f32>,
    pub l_age_limit_units: Option<String>,
    pub u_age_limit: Option<String>,
    pub u_age_limit_num: Option<f32>,
    pub u_age_limit_units: Option<String>,
    pub gender: Option<String>,
    pub inclusion: Option<String>,
    pub exclusion: Option<String>,
    pub patient_info_sheet: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Recruitment
{
    pub target_enrolment: Option<String>,
    pub total_final_enrolment: Option<String>,
    pub total_target: Option<String>,
    pub recruitment_start: Option<String>,
    pub recruitment_end: Option<String>,
    pub recruitment_start_status_override: Option<String>,
    pub recruitment_status_override: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StudyCentre
{
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Results
{
    pub publication_plan: Option<String>,
    pub intent_to_publish: Option<String>,
    pub publication_details: Option<String>,
    pub publication_stage: Option<String>,
    pub biomed_related: Option<bool>, 
    pub basic_report: Option<String>,
    pub plain_english_report: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct IPD
{
    pub ipd_sharing_plan: bool,    
    pub ipd_sharing_statement: Option<String>,

}

#[derive(Serialize, Deserialize)]
pub struct StudyLink
{
    pub link_type: Option<String>,
    pub link_url: Option<String>,
    pub description: Option<String>,
    pub date_created: Option<String>,
    pub date_uploaded: Option<String>,
}  

// Not serialised, but used as an intermediary struct
// when constructing A StudyFile

pub struct LocalFile
{
    pub file_type: Option<String>,
    pub file_id: Option<String>,
    pub description: Option<String>,
    pub download_filename: Option<String>,
    pub version: Option<String>,
    pub length: Option<i32>,
    pub mime_type: Option<String>,
    pub date_created: Option<String>,
    pub date_uploaded: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct StudyFile
{
    pub file_type: Option<String>,
    pub file_id: Option<String>,
    pub description: Option<String>,
    pub download_url: Option<String>,       
    pub download_filename: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub length: Option<i32>,
    pub mime_type: Option<String>,
    pub is_public: Option<bool>, 
    pub date_created: Option<String>,
    pub date_uploaded: Option<String>,
}
