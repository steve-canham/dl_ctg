pub mod monitoring;
pub mod process_study;

use crate::data_models::ctg_api::CTGStudy;
use crate::data_models::json_models::Study;
use crate::err::AppError;
use crate::base_types::{DownloadResult, InitParams};
use crate::setup::get_db_pool;
use sqlx::postgres::PgPool;
use std::path::PathBuf;
use std::fs;
use log::info;
use serde_json::to_string_pretty;
use std::io::Write;
use std::{thread, time};

pub async fn do_folder_download(dl_id: i32, params: &InitParams) -> Result<DownloadResult, AppError> {  

    // derive required parameters from parameters object

    let data_path = &params.source_data_path;
    let json_folder = &params.json_files_path;
   
    // Set up different DB pools. Which one will be needed for any
    // individual file will depend on when the registration was first created

    let db_pool1 =  get_db_pool("db1").await?;     
    let db_pool2 =  get_db_pool("db2").await?; 
    let db_pool3 =  get_db_pool("db3").await?; 

    let mut res = DownloadResult::new();

    // do each folder in turn

    for i in 18..100 {
       
        // Make a collection of PathBufs...

        let folder_num_as_string = format!("{:02}", i);
        let data_folder: PathBuf = [data_path, &PathBuf::from(&folder_num_as_string)].iter().collect();
        if !folder_exists (&data_folder) { 
            return Result::Err(AppError::ConfigurationError("Problem with parameters provided".to_string(), 
                        "Designated data folder does not exist".to_string()));
        }
        
        let paths = get_files_in_folder(&data_folder)?;  //  Grab file names from folder as vector of PathBufs
        
        info!("Number of files in folder {}: {}", format!("{:02}", i), paths.len());

        let mut path_res = DownloadResult::new();

        for path in paths {

            let file_contents = fs::read_to_string(&path)
                        .map_err(|e|AppError::IoReadErrorWithPath(e, path))?;

            let ctg_study: CTGStudy = serde_json::from_str(&file_contents).expect("JSON was not well-formatted");

            path_res.num_checked += 1;

            let study = process_study::process_study(ctg_study)?;

            let sid = study.sd_sid.clone();
            let file_name = format!("{}.json", sid.clone());
            let (datefp, post_year, post_month, yr) = get_first_post_params(&study, &file_name);

            let db_pool: &PgPool;
            if yr < 2016 {
                db_pool = &db_pool1;     
            }
            else if yr < 2023 {
                db_pool = &db_pool2;   
            }
            else {
                db_pool = &db_pool3   
            }

            // Derive these logging parameters from study before it is 
            // consumed by the call to write out the file.

            let sid = study.sd_sid.clone();
            let rec_last_revised = study.registration.date_last_updated.clone();

            let full_path: PathBuf = write_out_file (study, json_folder, &post_year, &post_month, &file_name)?;   
            path_res.num_downloaded += 1;

            if monitoring::update_ctg_mon(&sid, &datefp, &rec_last_revised, dl_id, 
                        &post_year, &post_month, &full_path, db_pool).await? {
                path_res.num_added += 1;
            }
       
        }

        info!("Folder {:02} processed", i);
        info!("Number of files in this folder checked: {}, downloaded: {}, added: {}", 
                                path_res.num_checked, path_res.num_downloaded, path_res.num_added);
        res = res + path_res;
        info!("Total number of files checked: {}, downloaded: {}, added: {}", 
                                res.num_checked, res.num_downloaded, res.num_added);
        info!("");

        let pause = time::Duration::from_secs(5);
        thread::sleep(pause);

    }

    Ok(res)

}


fn folder_exists(folder_name: &PathBuf) -> bool {
    match folder_name.try_exists() {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    }
}

fn get_files_in_folder(path: &PathBuf) -> Result<Vec<PathBuf>, AppError> {
    let entries = fs::read_dir(path)?;
    let files: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();
    Ok(files)
}

fn get_first_post_params(study: &Study, file_name: &String) -> (String, String, String, i32){

    // A correctly formatted ISO first post date should always be present, 
    // but just in case the relevant parameters are obtained here with default 
    // fall backs in the event of any problems.

    let mut datefp: String = match study.registration.date_first_posted {
        Some(ref d) => d.to_string(),
        None => {   // Not expected but just in case...
            info!("No date first posted found for study {}", file_name);
            info!("Study given default first posted date of 1998-01-01");
            "1998-01-01".to_string()
        },
    };
  
    if datefp.len() != 10 {     // Not expected but just in case...
        info!("Invalid date first posted, ({}), found for study {}", &datefp, file_name);
        info!("Study given default first posted date of 1998-01-01");
        datefp = "1998-01-01".to_string();
    }

    let date_year = datefp[0..4].to_string();
    let date_month = datefp[5..7].to_string();

    let yr = match date_year.parse::<i32>() {
            Ok(y) => y,
            Err(_) => {     // Not expected but just in case...
                info!("Unable to obtain year from date first posted, ({}), for study {}", &datefp, file_name);
                info!("Study given default first year of 1998");
                1998
            },
    };

    (datefp, date_year, date_month, yr)
}


fn write_out_file (study: Study, json_folder: &PathBuf, 
                    post_year: &str, post_month: &str, file_name: &String) -> Result<PathBuf, AppError>{

    let file_folder_path: PathBuf= [json_folder, &PathBuf::from(post_year), &PathBuf::from(post_month)].iter().collect();
    if !folder_exists (&file_folder_path) { 
        fs::create_dir_all(&file_folder_path)?;
    }
    
    let file_path: PathBuf = [&file_folder_path, &PathBuf::from(file_name)].iter().collect();
    let json_string = to_string_pretty(&study)
        .map_err(|e|AppError::SerdeError(e))?;
    let mut file = fs::File::create(&file_path)
        .map_err(|e|AppError::IoWriteErrorWithPath(e, file_path.clone()))?;
    file.write_all(json_string.as_bytes())
        .map_err(|e|AppError::IoWriteErrorWithPath(e, file_path.clone()))?;

    Ok(file_path)
}

