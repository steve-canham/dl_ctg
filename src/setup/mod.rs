pub mod config_reader;
pub mod log_helper;
pub mod cli_reader;

use crate::err::AppError;
use sqlx::postgres::{PgPoolOptions, PgConnectOptions, PgPool};
use std::path::PathBuf;
use cli_reader::{CliPars, Flags};
use std::fs;
use std::time::Duration;
use sqlx::ConnectOptions;
use config_reader::Config;
use std::sync::OnceLock;
//use chrono::NaiveDate;

pub struct InitParams {
    pub log_folder_path: PathBuf,
    pub json_files_path: PathBuf,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub flags: Flags,
}

pub static LOG_RUNNING: OnceLock<bool> = OnceLock::new();

pub fn get_params(cli_pars: CliPars, config_string: &String) -> Result<InitParams, AppError> {

    // Called from lib::run as the initial task of the program.
    // Returns a struct that contains the program's parameters.
    
    let config_file: Config = config_reader::populate_config_vars(&config_string)?; 

    // No overlap between parameters - CLI provides selection of program action
    // Config file provides folders and database parameters

    let log_folder = config_file.folders.log_folder_path;  
    if !folder_exists (&log_folder) { 
        fs::create_dir_all(&log_folder)?;
    }

    let json_folder = config_file.folders.json_files_path;  
    if !folder_exists (&log_folder) { 
        fs::create_dir_all(&json_folder)?;
    }
   
   
    // For execution flags read from the environment variables
    
    Ok(InitParams {
        log_folder_path: log_folder,
        json_files_path: json_folder,
        start_date: cli_pars.start_date,
        end_date: cli_pars.end_date,
        flags: cli_pars.flags,
    })

}


fn folder_exists(folder_name: &PathBuf) -> bool {
    let xres = folder_name.try_exists();
    let res = match xres {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}
        


pub async fn get_db_pool(db: &str) -> Result<PgPool, AppError> {  

    // Use DB name to get the connection string
    // Use the string to set up a connection options object and change 
    // the time threshold for warnings. Set up a DB pool option and 
    // connect using the connection options object.

    let db_name = config_reader::fetch_db_name(db)?;
    let db_conn_string = config_reader::fetch_db_conn_string(&db_name)?;  
   
    let mut opts: PgConnectOptions = db_conn_string.parse()
                    .map_err(|e| AppError::DBPoolError("Problem with parsing conection string".to_string(), e))?;
    opts = opts.log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(5));

    PgPoolOptions::new()
        .max_connections(5) 
        .connect_with(opts).await
        .map_err(|e| AppError::DBPoolError(format!("Problem with connecting to database {} and obtaining Pool", db_name), e))
}


pub fn establish_log(params: &InitParams) -> Result<(), AppError> {

    if !log_running() {  // can be called more than once in context of integration tests
        log_helper::setup_log(&params.log_folder_path)?;
        LOG_RUNNING.set(true).unwrap(); // should always work
        log_helper::log_startup_params(&params);
    }
    Ok(())
}

pub fn log_running() -> bool {
    match LOG_RUNNING.get() {
        Some(_) => true,
        None => false,
    }
}


// Tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::ffi::OsString;
  
    // No interaction between CLI and config file params
    // Therefore tests can be handled in cli_rewader and 
    // config_reader fiels

    #[test]
    fn check_a_flag_with_params() {
         let config = r#"
[folders]
log_folder_path="/home/steve/Data/MDR logs/ctg/"
json_files_path="/home/steve/Data/MDR json files/ctg/"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"

db1_name="ctg1"
db2_name="ctg2"
db3_name="ctg3"
mon_db_name="mon"
cxt_db_name="cxt"
 "#;
        let config_string = config.to_string();
        let args : Vec<&str> = vec!["dummy target", "-a", "-d", "2025-03-03"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res = get_params(cli_pars, &config_string).unwrap();

         assert_eq!(res.log_folder_path, PathBuf::from("/home/steve/Data/MDR logs/ctg/"));
         assert_eq!(res.json_files_path, PathBuf::from("/home/steve/Data/MDR json files/ctg/"));

         assert_eq!(res.start_date, Some("2025-03-03".to_string()));
         assert_eq!(res.end_date, None);
         assert_eq!(res.flags.download_recent, true);
         assert_eq!(res.flags.download_set, false);
         assert_eq!(res.flags.download_year, false);
         assert_eq!(res.flags.process_recent, true);
         assert_eq!(res.flags.process_set, false);   
         assert_eq!(res.flags.code_uncoded, true);
         assert_eq!(res.flags.code_all, false);           
         assert_eq!(res.flags.is_test, false);

    }
   
}

 