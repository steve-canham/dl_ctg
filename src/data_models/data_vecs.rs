use crate::data_models::db_models::*;
use crate::iec::iec_structs::IECLine;
use crate::AppError;
use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use chrono::{NaiveDate, NaiveDateTime};

pub struct StudyVecs {
    pub sd_sids: Vec<String>,
    pub display_titles: Vec<String>,
    pub brief_descriptions: Vec<String>,
    pub type_ids: Vec<i32>,
	pub status_ids: Vec<i32>,
    pub status_overrides: Vec<Option<String>>,
    pub start_status_overrides: Vec<Option<String>>,
    pub is_ipd_sharings: Vec<bool>,
	pub ipd_sharing_plans: Vec<Option<String>>,
    pub date_last_reviseds: Vec<Option<NaiveDate>>,
	pub dt_of_datas: Vec<NaiveDateTime>,

}

impl StudyVecs{
    pub fn new(vsize: usize) -> Self {
        StudyVecs { 
            sd_sids: Vec::with_capacity(vsize),
            display_titles: Vec::with_capacity(vsize),
            brief_descriptions: Vec::with_capacity(vsize),
            type_ids: Vec::with_capacity(vsize),
            status_ids: Vec::with_capacity(vsize),
            status_overrides: Vec::with_capacity(vsize),
            start_status_overrides: Vec::with_capacity(vsize),
            is_ipd_sharings: Vec::with_capacity(vsize),
	        ipd_sharing_plans: Vec::with_capacity(vsize),
            date_last_reviseds: Vec::with_capacity(vsize),
	        dt_of_datas: Vec::with_capacity(vsize),
        }
    }
    
    pub fn add(&mut self, sd_sid:&String, r: &DBSummary) 
    {
        self.sd_sids.push(sd_sid.clone());
        self.display_titles.push(r.display_title.clone());
        self.brief_descriptions.push(r.brief_description.clone());
        self.type_ids.push(r.type_id);
        self.status_ids.push(r.status_id);
        self.status_overrides.push(r.status_override.clone());
        self.start_status_overrides.push(r.start_status_override.clone());
        self.is_ipd_sharings.push(r.is_ipd_sharing);
        self.ipd_sharing_plans.push(r.ipd_sharing_plan.clone());
        self.date_last_reviseds.push(r.date_last_revised);
        self.dt_of_datas.push(r.dt_of_data_fetch);
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.studies (sd_sid, display_title, brief_description, type_id, status_id, status_override, start_status_override,
                        is_ipd_sharing, ipd_sharing_plan, date_last_revised, dt_of_data_fetch) 
                        SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::int[], $5::int[], $6::text[], $7::text[], 
                        $8::bool[], $9::text[], $10::date[], $11::timestamp[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.display_titles)
        .bind(&self.brief_descriptions)
        .bind(&self.type_ids)
        .bind(&self.status_ids)
        .bind(&self.status_overrides)
        .bind(&self.start_status_overrides)
        .bind(&self.is_ipd_sharings)
        .bind(&self.ipd_sharing_plans)
        .bind(&self.date_last_reviseds)
        .bind(&self.dt_of_datas)

        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

pub struct StudyDatesVecs {
    pub sd_sids: Vec<String>,
    pub reg_years: Vec<Option<i32>>,  
	pub reg_months: Vec<Option<i32>>,
    pub reg_date_types: Vec<Option<String>>,         
	pub start_years: Vec<Option<i32>>, 
	pub start_months: Vec<Option<i32>>,   
    pub start_date_types: Vec<Option<String>>,       
	pub comp_years: Vec<Option<i32>>,
	pub comp_months: Vec<Option<i32>>,  
	pub comp_date_types: Vec<Option<String>>,      
	pub res_years: Vec<Option<i32>>,  
	pub res_months: Vec<Option<i32>>,   
    pub res_date_types: Vec<Option<String>>,    
}

impl StudyDatesVecs{
    pub fn new(vsize: usize) -> Self {
        StudyDatesVecs { 
            sd_sids: Vec::with_capacity(vsize),
            reg_years: Vec::with_capacity(vsize),  
            reg_months: Vec::with_capacity(vsize),
            reg_date_types: Vec::with_capacity(vsize),        
            start_years: Vec::with_capacity(vsize),
            start_months: Vec::with_capacity(vsize),   
            start_date_types: Vec::with_capacity(vsize),       
            comp_years: Vec::with_capacity(vsize),
            comp_months: Vec::with_capacity(vsize),
            comp_date_types: Vec::with_capacity(vsize),     
            res_years: Vec::with_capacity(vsize), 
            res_months: Vec::with_capacity(vsize),   
            res_date_types: Vec::with_capacity(vsize), 
        }
    }

    pub fn add(&mut self, sd_sid:&String, r: &DBStudyDates) 
    {
        self.sd_sids.push(sd_sid.clone());
        self.reg_years.push(r.reg_year);
        self.reg_months.push(r.reg_month);
        self.reg_date_types.push(r.reg_date_type.clone());
        self.start_years.push(r.start_year);
        self.start_months.push(r.start_month);
        self.start_date_types.push(r.start_date_type.clone());
        self.comp_years.push(r.comp_year);
        self.comp_months.push(r.comp_month);
        self.comp_date_types.push(r.comp_date_type.clone());
        self.res_years.push(r.res_year);
        self.res_months.push(r.res_month);
        self.res_date_types.push(r.res_date_type.clone());
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_dates (sd_sid, reg_year, reg_month, reg_date_type, start_year, start_month, start_date_type,
                        comp_year, comp_month, comp_date_type, res_year, res_month, res_date_type) 
            SELECT * FROM UNNEST($1::text[], $2::int[], $3::int[], $4::text[], $5::int[], $6::int[], $7::text[], 
                                 $8::int[], $9::int[], $10::text[], $11::int[], $12::int[], $13::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.reg_years)
        .bind(&self.reg_months)
        .bind(&self.reg_date_types)
        .bind(&self.start_years)
        .bind(&self.start_months)
        .bind(&self.start_date_types)
        .bind(&self.comp_years)
        .bind(&self.comp_months)
        .bind(&self.comp_date_types)
        .bind(&self.res_years)
        .bind(&self.res_months)
        .bind(&self.res_date_types)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

pub struct StudyParticsVecs {
    pub sd_sids: Vec<String>,
    pub enrolment_targets: Vec<Option<String>>, 
    pub enrolment_finals: Vec<Option<String>>, 
    pub enrolment_totals: Vec<Option<String>>, 
    pub enrolments: Vec<Option<String>>, 
	pub enrolment_types: Vec<Option<String>>,
    pub gender_strings: Vec<Option<String>>,
	pub gender_flags: Vec<Option<String>>,
    pub min_age_strings: Vec<Option<String>>,
	pub min_ages: Vec<Option<f64>>,  
	pub min_age_units_ids: Vec<Option<String>>,
    pub max_age_strings: Vec<Option<String>>,
	pub max_ages: Vec<Option<f64>>,  
	pub max_age_units_ids: Vec<Option<String>>, 
	pub age_group_flags: Vec<i32>, 
    pub iec_flags: Vec<i32>,
}

impl StudyParticsVecs{
    pub fn new(vsize: usize) -> Self {
        StudyParticsVecs { 
            sd_sids: Vec::with_capacity(vsize),
            enrolment_targets: Vec::with_capacity(vsize), 
            enrolment_finals: Vec::with_capacity(vsize),
            enrolment_totals: Vec::with_capacity(vsize),
            enrolments: Vec::with_capacity(vsize),
            enrolment_types: Vec::with_capacity(vsize),
            gender_strings: Vec::with_capacity(vsize),
            gender_flags: Vec::with_capacity(vsize),
            min_age_strings: Vec::with_capacity(vsize),
            min_ages: Vec::with_capacity(vsize), 
            min_age_units_ids: Vec::with_capacity(vsize),
            max_age_strings: Vec::with_capacity(vsize),
            max_ages: Vec::with_capacity(vsize),  
            max_age_units_ids: Vec::with_capacity(vsize), 
            age_group_flags: Vec::with_capacity(vsize),
            iec_flags: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, r: &DBStudyPartics) 
    {
        self.sd_sids.push(sd_sid.clone());
        self.enrolment_targets.push(r.enrolment_target.clone());
        self.enrolment_finals.push(r.enrolment_final.clone());
        self.enrolment_totals.push(r.enrolment_total.clone());
        self.enrolments.push(r.enrolment.clone());
        self.enrolment_types.push(r.enrolment_type.clone());
        self.gender_strings.push(r.gender_string.clone());
        self.gender_flags.push(r.gender_flag.clone());
        self.min_age_strings.push(r.min_age_string.clone());
        self.min_ages.push(r.min_age);
        self.min_age_units_ids.push(r.min_age_units_id.clone());
        self.max_age_strings.push(r.max_age_string.clone());
        self.max_ages.push(r.max_age);
        self.max_age_units_ids.push(r.max_age_units_id.clone());
        self.age_group_flags.push(r.age_group_flag);
        self.iec_flags.push(r.iec_flag);

    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_participants (sd_sid, enrolment_target, enrolment_final,
                enrolment_total, enrolment, enrolment_type, gender_string, gender_flag,  
                min_age_string, min_age, min_age_units_id, 
                max_age_string, max_age, max_age_units_id, age_group_flag, iec_flag) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[], 
                    $7::text[], $8::text[], $9::text[], 
                    $10::float[], $11::text[], $12::text[], $13::float[], $14::text[], 
                    $15::int[], $16::int[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.enrolment_targets)
        .bind(&self.enrolment_finals)
        .bind(&self.enrolment_totals)
        .bind(&self.enrolments)
        .bind(&self.enrolment_types)
        .bind(&self.gender_strings)
        .bind(&self.gender_flags)
        .bind(&self.min_age_strings)
        .bind(&self.min_ages)
        .bind(&self.min_age_units_ids)
        .bind(&self.max_age_strings)
        .bind(&self.max_ages)
        .bind(&self.max_age_units_ids)
        .bind(&self.age_group_flags)
        .bind(&self.iec_flags)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }

}

pub struct TitleVecs {
    pub sd_sids: Vec<String>,
    pub title_texts: Vec<String>,
    pub is_publics: Vec<bool>,
    pub is_scientifics: Vec<bool>,
    pub is_acronyms: Vec<bool>,
    pub is_displays: Vec<bool>,
    pub comments: Vec<Option<String>>,
}

impl TitleVecs{
    pub fn new(vsize: usize) -> Self {
        TitleVecs { 
            sd_sids: Vec::with_capacity(vsize),
            title_texts: Vec::with_capacity(vsize),
            is_publics: Vec::with_capacity(vsize),
            is_scientifics: Vec::with_capacity(vsize),
            is_acronyms: Vec::with_capacity(vsize), 
            is_displays: Vec::with_capacity(vsize),         
            comments: Vec::with_capacity(vsize),
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.title_texts.shrink_to_fit();
        self.is_publics.shrink_to_fit();
        self.is_scientifics.shrink_to_fit();
        self.is_acronyms.shrink_to_fit();  
        self.is_displays.shrink_to_fit();     
        self.comments.shrink_to_fit();
    }


    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBTitle>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.title_texts.push(r.title_text.clone());
            self.is_publics.push(r.is_public);
            self.is_scientifics.push(r.is_scientific);
            self.is_acronyms.push(r.is_acronym);
            self.is_displays.push(r.is_display);
            self.comments.push(r.comment.clone());
        }
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_titles (sd_sid, title_text, is_public, is_scientific, is_acronym, is_display, comments) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::bool[], $4::bool[], $5::bool[], $6::bool[], $7::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.title_texts)
        .bind(&self.is_publics)
        .bind(&self.is_scientifics)
        .bind(&self.is_acronyms)
        .bind(&self.is_displays)       
        .bind(&self.comments)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


pub struct IdentifierVecs {
    pub sd_sids: Vec<String>,
    pub id_values: Vec<String>,
    pub id_type_ids: Vec<i32>,
    pub id_types: Vec<String>,
}

impl IdentifierVecs{
    pub fn new(vsize: usize) -> Self {
        IdentifierVecs { 
            sd_sids: Vec::with_capacity(vsize),
            id_values: Vec::with_capacity(vsize),
            id_type_ids: Vec::with_capacity(vsize),
            id_types: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBIdentifier>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.id_values.push(r.id_value.clone());
            self.id_type_ids.push(r.id_type_id);
            self.id_types.push(r.id_type.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.id_values.shrink_to_fit();
        self.id_type_ids.shrink_to_fit();
        self.id_types.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_identifiers (sd_sid, id_value, id_type_id, id_type) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::int[], $4::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.id_values)
        .bind(&self.id_type_ids)
        .bind(&self.id_types)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


#[allow(dead_code)]
pub struct OrgVecs {
    pub sd_sids: Vec<String>,
    pub org_names: Vec<Option<String>>,
    pub is_sponsors: Vec<Option<bool>>,   
    pub is_funders: Vec<Option<bool>>,  
    pub is_collaborators: Vec<Option<bool>>,  
    pub org_countries: Vec<Option<String>>,
    pub org_ror_ids: Vec<Option<String>>,
    pub org_cref_ids: Vec<Option<String>>,
}

impl OrgVecs{
    pub fn new(vsize: usize) -> Self {
        OrgVecs { 
            sd_sids: Vec::with_capacity(vsize),
            org_names: Vec::with_capacity(vsize),
            is_sponsors: Vec::with_capacity(vsize),   
            is_funders: Vec::with_capacity(vsize),
            is_collaborators: Vec::with_capacity(vsize), 
            org_countries: Vec::with_capacity(vsize),
            org_ror_ids: Vec::with_capacity(vsize),
            org_cref_ids: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBOrganisation>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.org_names.push(r.org_name.clone());
            self.is_sponsors.push(r.is_sponsor); 
            self.is_funders.push(r.is_funder);
            self.is_collaborators.push(r.is_collaborator);
            self.org_countries.push(r.org_country.clone());
            self.org_ror_ids.push(r.org_ror_id.clone());
            self.org_cref_ids.push(r.org_cref_id.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.org_names.shrink_to_fit();
        self.is_sponsors.shrink_to_fit();
        self.is_funders.shrink_to_fit();
        self.is_collaborators.shrink_to_fit();
        self.org_countries.shrink_to_fit();
        self.org_ror_ids.shrink_to_fit();
        self.org_cref_ids.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_orgs (sd_sid, org_name, is_sponsor, is_funder, 
                          is_collaborator, org_country, org_ror_id, org_cref_id) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::bool[], $4::bool[], $5::bool[], 
                     $6::text[], $7::text[], $8::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.org_names)
        .bind(&self.is_sponsors)
        .bind(&self.is_funders)
        .bind(&self.is_collaborators)
        .bind(&self.org_countries)
        .bind(&self.org_ror_ids)
        .bind(&self.org_cref_ids)

        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

pub struct PeopleVecs {
    pub sd_sids: Vec<String>,
    pub full_names: Vec<Option<String>>,
    pub listed_ass: Vec<Option<String>>,
    pub orcid_ids: Vec<Option<String>>,
    pub affiliations: Vec<Option<String>>,
    pub email_domains: Vec<Option<String>>,
}

impl PeopleVecs{
    pub fn new(vsize: usize) -> Self {
        PeopleVecs { 
            sd_sids: Vec::with_capacity(vsize),
            full_names: Vec::with_capacity(vsize),
            listed_ass: Vec::with_capacity(vsize),  
            orcid_ids: Vec::with_capacity(vsize),
            affiliations: Vec::with_capacity(vsize),
            email_domains: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBPerson>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.full_names.push(r.full_name.clone());
            self.listed_ass.push(r.listed_as.clone()); 
            self.orcid_ids.push(r.orcid_id.clone());
            self.affiliations.push(r.affiliation.clone());
            self.email_domains.push(r.email_domain.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.full_names.shrink_to_fit();
        self.listed_ass.shrink_to_fit();
        self.orcid_ids.shrink_to_fit();
        self.affiliations.shrink_to_fit();
        self.email_domains.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_people (sd_sid, full_name, listed_as,
                        orcid_id, affiliation, email_domain) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], 
                        $4::text[], $5::text[], $6::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.full_names)
        .bind(&self.listed_ass)
        .bind(&self.orcid_ids)
        .bind(&self.affiliations)
        .bind(&self.email_domains)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

pub struct LocationVecs {
    pub sd_sids: Vec<String>,
    pub fac_names: Vec<Option<String>>,
    pub fac_addresses: Vec<Option<String>>,
    pub city_names: Vec<Option<String>>,
    pub disamb_names: Vec<Option<String>>,
    pub country_names: Vec<Option<String>>,
}

impl LocationVecs{
    pub fn new(vsize: usize) -> Self {
        LocationVecs { 
            sd_sids: Vec::with_capacity(vsize),
            fac_names: Vec::with_capacity(vsize),
            fac_addresses: Vec::with_capacity(vsize),
            city_names: Vec::with_capacity(vsize),
            disamb_names: Vec::with_capacity(vsize),
            country_names: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBLocation>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.fac_names.push(r.fac_name.clone());
            self.fac_addresses.push(r.fac_address.clone());
            self.city_names.push(r.city_name.clone());
            self.disamb_names.push(r.disamb_name.clone());
            self.country_names.push(r.country_name.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.fac_names.shrink_to_fit();
        self.fac_addresses.shrink_to_fit();
        self.city_names.shrink_to_fit();
        self.disamb_names.shrink_to_fit();
        self.country_names.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_locations (sd_sid, fac_name, fac_address, city_name, disamb_name, country_name) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.fac_names)
        .bind(&self.fac_addresses)
        .bind(&self.city_names)
        .bind(&self.disamb_names)
        .bind(&self.country_names)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

pub struct CountryVecs {
    pub sd_sids: Vec<String>,
    pub country_names: Vec<String>,
}


impl CountryVecs{
    pub fn new(vsize: usize) -> Self {
        CountryVecs { 
            sd_sids: Vec::with_capacity(vsize),
            country_names: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBCountry>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.country_names.push(r.country_name.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.country_names.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_countries (sd_sid, country_name) 
            SELECT * FROM UNNEST($1::text[], $2::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.country_names)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

pub struct ConditionVecs {
    pub sd_sids: Vec<String>,
    pub class1s: Vec<Option<String>>,
    pub class2s: Vec<Option<String>>,
    pub specifics: Vec<Option<String>>,
}

impl ConditionVecs {

    pub fn new(vsize: usize) -> Self {
        ConditionVecs { 
            sd_sids: Vec::with_capacity(vsize),
            class1s: Vec::with_capacity(vsize),
            class2s: Vec::with_capacity(vsize),
            specifics: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBCondition>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.class1s.push(r.class1.clone());
            self.class2s.push(r.class2.clone());
            self.specifics.push(r.specific.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.class1s.shrink_to_fit();
        self.class2s.shrink_to_fit();
        self.specifics.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_conditions (sd_sid, class1, class2, specific) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.class1s)
        .bind(&self.class2s)
        .bind(&self.specifics)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

pub struct FeatureVecs {
    pub sd_sids: Vec<String>,
    pub sources: Vec<String>,
    pub feature_types: Vec<String>,
    pub feature_values: Vec<String>,
}

impl FeatureVecs{
    pub fn new(vsize: usize) -> Self {
        FeatureVecs { 
            sd_sids: Vec::with_capacity(vsize),
            sources: Vec::with_capacity(vsize),
            feature_types: Vec::with_capacity(vsize),
            feature_values: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBFeature>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.sources.push(r.source.clone());
            self.feature_types.push(r.feature_type.clone());
            self.feature_values.push(r.feature_value.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.sources.shrink_to_fit();
        self.feature_types.shrink_to_fit();
        self.feature_values.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_features(sd_sid, source, feature_type, feature_value) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.sources)
        .bind(&self.feature_types)
        .bind(&self.feature_values)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

pub struct TopicVecs {
    pub sd_sids: Vec<String>,
    pub sources: Vec<String>,  
    pub topic_types: Vec<String>,
    pub topic_values: Vec<String>,
}

impl TopicVecs{
    pub fn new(vsize: usize) -> Self {
        TopicVecs { 
            sd_sids: Vec::with_capacity(vsize),
            sources: Vec::with_capacity(vsize),
            topic_types: Vec::with_capacity(vsize),
            topic_values: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBTopic>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.sources.push(r.source.clone());
            self.topic_types.push(r.topic_type.clone());
            self.topic_values.push(r.topic_value.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.sources.shrink_to_fit();
        self.topic_types.shrink_to_fit();
        self.topic_values.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_topics (sd_sid, source, topic_type, topic_value) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.sources)
        .bind(&self.topic_types)
        .bind(&self.topic_values)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

pub struct IECVecs {
    pub sd_sids: Vec<String>,
    pub seq_nums:  Vec<i32>,
    pub ie_type_ids:  Vec<i32>,
    pub tag_types: Vec<String>,
    pub tags: Vec<String>,
    pub indent_levels: Vec<i32>,
    pub indent_seq_nums: Vec<i32>,
    pub sequence_strings: Vec<String>,
    pub criteria: Vec<String>,
}

impl IECVecs{
    pub fn new(vsize: usize) -> Self {
        IECVecs { 
            sd_sids: Vec::with_capacity(vsize),
            seq_nums: Vec::with_capacity(vsize),
            ie_type_ids: Vec::with_capacity(vsize),
            tag_types: Vec::with_capacity(vsize),
            tags: Vec::with_capacity(vsize),
            indent_levels: Vec::with_capacity(vsize),
            indent_seq_nums: Vec::with_capacity(vsize),
            sequence_strings: Vec::with_capacity(vsize),
            criteria: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<IECLine>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.seq_nums.push(r.seq_num);
            self.ie_type_ids.push(r.type_id);
            self.tag_types.push(r.tag_type.clone());
            self.tags.push(r.tag.clone());
            self.indent_levels.push(r.indent_level);
            self.indent_seq_nums.push(r.indent_seq_num);
            self.sequence_strings.push(r.sequence_string.clone());
            self.criteria.push(r.text.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
       
            self.sd_sids.shrink_to_fit();
            self.seq_nums.shrink_to_fit();
            self.ie_type_ids.shrink_to_fit();
            self.tag_types.shrink_to_fit();
            self.tags.shrink_to_fit();
            self.indent_levels.shrink_to_fit();
            self.indent_seq_nums.shrink_to_fit();
            self.sequence_strings.shrink_to_fit();
            self.criteria.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_iec (sd_sid, seq_num, ie_type_id, split_type, tag, indent_level, indent_seq_num, sequence_string, criterion) 
            SELECT * FROM UNNEST($1::text[], $2::int[], $3::int[], $4::text[], $5::text[], $6::int[], $7::int[], $8::text[], $9::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.seq_nums)
        .bind(&self.ie_type_ids)
        .bind(&self.tag_types)
        .bind(&self.tags)
        .bind(&self.indent_levels)
        .bind(&self.indent_seq_nums)
        .bind(&self.sequence_strings)
        .bind(&self.criteria)

        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


pub struct ObjectVecs {
    pub sd_sids: Vec<String>,
    pub object_types: Vec<String>,
    pub object_ids: Vec<String>,   
    pub object_id_types: Vec<String>,
    pub object_notess: Vec<Option<String>>,
    pub display_names: Vec<Option<String>>,
    pub access_urls: Vec<Option<String>>,
    pub access_types: Vec<Option<String>>,
    pub instance_types: Vec<Option<String>>,
    pub instance_notess: Vec<Option<String>>,
    pub date_createds: Vec<Option<NaiveDate>>,
    pub date_uploadeds: Vec<Option<NaiveDate>>,
}

impl ObjectVecs{
    pub fn new(vsize: usize) -> Self {
        ObjectVecs { 
            sd_sids: Vec::with_capacity(vsize),
            object_types: Vec::with_capacity(vsize),
            object_ids: Vec::with_capacity(vsize),   
            object_id_types: Vec::with_capacity(vsize),
            object_notess: Vec::with_capacity(vsize),
            display_names: Vec::with_capacity(vsize),
            access_urls: Vec::with_capacity(vsize),
            access_types: Vec::with_capacity(vsize),
            instance_types:  Vec::with_capacity(vsize),
            instance_notess: Vec::with_capacity(vsize),
            date_createds: Vec::with_capacity(vsize),
            date_uploadeds: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBObject>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.object_types.push(r.object_type.clone());
            self.object_ids.push(r.object_id.clone());
            self.object_id_types.push(r.object_id_type.clone());
            self.object_notess.push(r.object_notes.clone());
            self.display_names.push(r.display_name.clone());
            self.access_urls.push(r.access_url.clone());
            self.access_types.push(r.access_type.clone());
            self.instance_types.push(r.instance_type.clone());
            self.instance_notess.push(r.instance_notes.clone());
            self.date_createds.push(r.date_created.clone());
            self.date_uploadeds.push(r.date_uploaded.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
       
            self.sd_sids.shrink_to_fit();
            self.object_types.shrink_to_fit();
            self.object_ids.shrink_to_fit();
            self.object_id_types.shrink_to_fit();
            self.object_notess.shrink_to_fit();
            self.display_names.shrink_to_fit();
            self.access_urls.shrink_to_fit();
            self.access_types.shrink_to_fit();
            self.instance_types.shrink_to_fit();
            self.instance_notess.shrink_to_fit();
            self.date_createds.shrink_to_fit();
            self.date_uploadeds.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_objects (sd_sid, object_type, object_id,
                         object_id_type, object_notes, display_name, access_url, 
                         access_type, instance_type, instance_notes, date_created, date_published) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[], 
                         $7::text[], $8::text[], $9::text[], $10::text[], $11::date[], $12::date[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.object_types)
        .bind(&self.object_ids)
        .bind(&self.object_id_types)
        .bind(&self.object_notess)
        .bind(&self.display_names)
        .bind(&self.access_urls)
        .bind(&self.access_types)
        .bind(&self.instance_types)
        .bind(&self.instance_notess)
        .bind(&self.date_createds)
        .bind(&self.date_uploadeds)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}



pub struct PublicationVecs {
    pub sd_sids: Vec<String>,
    pub pub_types: Vec<String>,
    pub pub_ids: Vec<String>,
    pub pub_id_types: Vec<String>,
    pub pub_notes: Vec<Option<String>>,
    pub external_urls: Vec<Option<String>>,
    pub dois: Vec<Option<String>>,
    pub pmids: Vec<Option<String>>,
    pub pmcids: Vec<Option<String>>,
    pub pubsite_urls: Vec<Option<String>>,
    pub date_createds: Vec<Option<NaiveDate>>,
    pub date_uploadeds: Vec<Option<NaiveDate>>,


}


impl PublicationVecs{
    pub fn new(vsize: usize) -> Self {
        PublicationVecs { 
            sd_sids: Vec::with_capacity(vsize),
            pub_types: Vec::with_capacity(vsize),
            pub_ids: Vec::with_capacity(vsize),
            pub_id_types: Vec::with_capacity(vsize),
            pub_notes: Vec::with_capacity(vsize),
            external_urls: Vec::with_capacity(vsize),
            dois: Vec::with_capacity(vsize),
            pmids: Vec::with_capacity(vsize),
            pmcids: Vec::with_capacity(vsize),
            pubsite_urls: Vec::with_capacity(vsize),
            date_createds: Vec::with_capacity(vsize),
            date_uploadeds: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBPublication>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.pub_types.push(r.pub_type.clone());
            self.pub_ids.push(r.pub_id.clone());
            self.pub_id_types.push(r.pub_id_type.clone());
            self.pub_notes.push(r.pub_notes.clone());
            self.external_urls.push(r.external_url.clone());
            self.dois.push(r.doi.clone());
            self.pmids.push(r.pmid.clone());
            self.pmcids.push(r.pmcid.clone());
            self.pubsite_urls.push(r.pubsite_url.clone());
            self.date_createds.push(r.date_created.clone());
            self.date_uploadeds.push(r.date_uploaded.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
       
            self.sd_sids.shrink_to_fit();
            self.pub_types.shrink_to_fit();
            self.pub_ids.shrink_to_fit();
            self.pub_id_types.shrink_to_fit();
            self.pub_notes.shrink_to_fit();
            self.external_urls.shrink_to_fit();
            self.dois.shrink_to_fit();
            self.pmids.shrink_to_fit();
            self.pmcids.shrink_to_fit();
            self.pubsite_urls.shrink_to_fit();
            self.date_createds.shrink_to_fit();
            self.date_uploadeds.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_pubs (sd_sid, pub_type, pub_id, pub_id_type, pub_notes, external_url,
                         doi, pmid, pmcid, pubsite_url, date_created, date_published) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], 
                         $6::text[], $7::text[], $8::text[], $9::text[], $10::text[], $11::date[], $12::date[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.pub_types)
        .bind(&self.pub_ids)
        .bind(&self.pub_id_types)
        .bind(&self.pub_notes)
        .bind(&self.external_urls)
        .bind(&self.dois)
        .bind(&self.pmids)
        .bind(&self.pmcids)
        .bind(&self.pubsite_urls)
        .bind(&self.date_createds)
        .bind(&self.date_uploadeds)


        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

