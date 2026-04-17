use crate::iec::iec_structs::IECLine;
use chrono::{NaiveDate, NaiveDateTime};


pub struct DBStudy {

    pub sd_sid: String,
    pub summary: DBSummary,
    pub dates: DBStudyDates,
    pub participants: DBStudyPartics,
    pub titles: Option<Vec<DBTitle>>, 
    pub identifiers: Option<Vec<DBIdentifier>>,
    pub orgs: Option<Vec<DBOrganisation>>, 
    pub people: Option<Vec<DBPerson>>, 
    pub locations: Option<Vec<DBLocation>>, 
    pub countries: Option<Vec<DBCountry>>, 
    pub conditions: Option<Vec<DBCondition>>, 
    pub features: Option<Vec<DBFeature>>, 
    pub topics: Option<Vec<DBTopic>>, 
    pub ie_crit: Option<Vec<IECLine>>, 
    pub publications: Option<Vec<DBPublication>>,
    pub objects: Option<Vec<DBObject>>,
}

pub struct DBSummary {
    pub display_title: String,
    pub brief_description: String,

    pub type_id: i32,
	pub status_id: i32,

    pub status_override: Option<String>,
    pub start_status_override: Option<String>,

    pub is_ipd_sharing: bool,
	pub ipd_sharing_plan: Option<String>,
    pub date_last_revised: Option<NaiveDate>,
	pub dt_of_data_fetch: NaiveDateTime,
}

pub struct DBStudyDates {
    pub reg_year: Option<i32>,  
	pub reg_month: Option<i32>,
    pub reg_date_type: Option<String>,         
	pub start_year: Option<i32>, 
	pub start_month: Option<i32>,   
    pub start_date_type: Option<String>,       
	pub comp_year: Option<i32>,
	pub comp_month: Option<i32>,  
	pub comp_date_type: Option<String>,      
	pub res_year: Option<i32>,  
	pub res_month: Option<i32>,   
    pub res_date_type: Option<String>,    
}

pub struct DBStudyPartics {

    pub enrolment_target: Option<String>, 
    pub enrolment_final: Option<String>, 
    pub enrolment_total: Option<String>, 

    pub enrolment: Option<String>, 
	pub enrolment_type: Option<String>,
    pub gender_string: Option<String>,  
	pub gender_flag: Option<String>,
    pub min_age_string: Option<String>,  
	pub min_age: Option<f64>,  
	pub min_age_units_id: Option<String>,
    pub max_age_string: Option<String>,  
	pub max_age: Option<f64>,  
	pub max_age_units_id: Option<String>, 
	pub age_group_flag: i32, 
    pub iec_flag: i32,
}

pub struct DBTitle {
    pub title_text: String,
    pub is_public: bool,
    pub is_scientific: bool,
    pub is_acronym: bool,
    pub is_display: bool,
    pub comment: Option<String>,
}

pub struct DBIdentifier {
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

pub struct DBOrganisation {
    pub org_name: Option<String>,
    pub is_sponsor: Option<bool>,   
    pub is_funder: Option<bool>,  
    pub is_collaborator: Option<bool>,  
    pub org_country: Option<String>,
    pub org_ror_id: Option<String>,
    pub org_cref_id: Option<String>,
}

pub struct DBPerson {
    pub full_name: Option<String>,
    pub listed_as: Option<String>,
    pub orcid_id: Option<String>,
    pub affiliation: Option<String>,
    pub email_domain: Option<String>,
}
    
pub struct DBLocation {
    pub fac_name: Option<String>,
    pub fac_address: Option<String>,
    pub city_name: Option<String>,
    pub disamb_name: Option<String>,
    pub country_name: Option<String>,
}

pub struct DBCountry {
    pub country_name: String,
}

pub struct DBTopic {
    pub source:  String,
    pub topic_type:  String,
    pub topic_value: String,
}

pub struct DBCondition {
    pub class1: Option<String>,
    pub class2: Option<String>,
    pub specific: Option<String>,
}

pub struct DBFeature {
    pub source: String,
    pub feature_type: String,
    pub feature_value: String,
}

pub struct DBPublication {
    pub pub_type: String,
    pub pub_id: String,
    pub pub_id_type: String,
    pub pub_notes: Option<String>,
    pub external_url: Option<String>,
    pub doi: Option<String>,
    pub pmid: Option<String>,
    pub pmcid: Option<String>,
    pub pubsite_url: Option<String>,
    pub date_created: Option<NaiveDate>,
    pub date_uploaded: Option<NaiveDate>,
}


pub struct DBObject {
    pub object_type: String,
    pub object_id: String,
    pub object_id_type: String,
    pub object_notes: Option<String>,
    pub display_name: Option<String>,
    pub access_url: Option<String>,
    pub access_type: Option<String>,
    pub instance_type: Option<String>,
    pub instance_notes: Option<String>,
    pub date_created: Option<NaiveDate>,
    pub date_uploaded: Option<NaiveDate>,
}



