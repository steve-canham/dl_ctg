pub mod monitoring;

use crate::data_models::ctg_api::CTGStudy;
use crate::err::AppError;
use crate::base_types::DownloadResult;
use crate::setup::get_db_pool;
use sqlx::postgres::PgPool;
use std::path::PathBuf;
use std::fs;
use log::info;

pub async fn do_year_download(_data_path: &PathBuf, start_date: &Option<String>) -> Result<DownloadResult, AppError> {  

    // Grab some file names from folder
    // Make a colletion of PathBufs...
    // Initially just use a test file...
    // Go through the first few to see if JSON model works

    let test_path = PathBuf::from("/home/steve/Data/MDR source data/CTGDumps/20260413/ctg-studies.json/00/NCT00000500.json");
    
    let file_contents = fs::read_to_string(&test_path)
                .map_err(|e|AppError::IoReadErrorWithPath(e, test_path))?;

    //info!("{}", file_contents);

    let study: CTGStudy = serde_json::from_str(&file_contents).expect("JSON was not well-formatted");

    info!("{}", study.protocol_section.identification_module.nct_id);

    let posted_date = study.protocol_section.status_module.study_posted_date.date.unwrap();

    info!("{}", posted_date);


    // start date already checked as a valid 4 digit string.
    // db and therefore pool required will depend upon the year...

    let post_year = &posted_date[0..4];
    let yr = post_year.parse::<i32>()
            .map_err(|e| AppError::ParseError(e))?;
    
    let _db_pool: PgPool;
    if yr < 2016 {
        _db_pool =  get_db_pool("db1").await?;     
    }
    else if yr < 2023 {
        _db_pool =  get_db_pool("db2").await?; 
    }
    else {
        _db_pool =  get_db_pool("db3").await?; 
    }

    info!("{}", yr);
        
    let res = DownloadResult::new();
 
    Ok(res)

}