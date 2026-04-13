
 use crate::AppError;
 use std::sync::OnceLock;
 use toml;
 use serde::Deserialize;
 use std::path::PathBuf;
 
 #[derive(Debug, Deserialize)]
 pub struct TomlConfig {
    pub folders: Option<TomlFolderPars>, 
    pub database: Option<TomlDBPars>,
 }
 
 #[derive(Debug, Deserialize)]
 pub struct TomlFolderPars {
    pub log_folder_path: Option<String>,
    pub json_files_path: Option<String>,
    pub source_data_path: Option<String>,
 }
 
 #[derive(Debug, Deserialize)]
 pub struct TomlDBPars {
    pub db_host: Option<String>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
    pub db_port: Option<String>,

    pub db1_name: Option<String>,
    pub db2_name: Option<String>,
    pub db3_name: Option<String>,
    pub cxt_db_name: Option<String>,
    pub mon_db_name: Option<String>,
 }
 
 pub struct Config {
    pub folders: FolderPars, 
    pub db_pars: DBPars,
 }

 pub struct FolderPars {
    pub log_folder_path: PathBuf,
    pub json_files_path: PathBuf,
    pub source_data_path: PathBuf,
 }
 
 #[derive(Debug, Clone)]
 pub struct DBPars {
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_port: usize,

    pub db1_name: String,
    pub db2_name: String,
    pub db3_name: String,
    pub cxt_db_name: String,
    pub mon_db_name: String,
 }
 

 pub static DB_PARS: OnceLock<DBPars> = OnceLock::new();
 
 pub fn populate_config_vars(config_string: &String) -> Result<Config, AppError> {
     
    let toml_config = toml::from_str::<TomlConfig>(&config_string)
         .map_err(|_| {AppError::ConfigurationError("Unable to parse config file.".to_string(),
         "File (app_config.toml) may be malformed.".to_string())})?;
            
    let toml_database = match toml_config.database {
         Some(d) => d,
         None => {return Result::Err(AppError::ConfigurationError("Missing or misspelt configuration section.".to_string(),
             "Cannot find a section called '[database]'.".to_string()))},
    };
 
    let toml_folders = match toml_config.folders {
         Some(f) => f,
         None => {return Result::Err(AppError::ConfigurationError("Missing or misspelt configuration section.".to_string(),
            "Cannot find a section called '[files]'.".to_string()))},
    };
    
    let config_folders = verify_folder_parameters(toml_folders)?;
    let config_db_pars = verify_db_parameters(toml_database)?;
 
    let _ = DB_PARS.set(config_db_pars.clone());
 
    Ok(Config{
         folders: config_folders,
         db_pars: config_db_pars,
     })
 }
 


 fn verify_folder_parameters(toml_folders: TomlFolderPars) -> Result<FolderPars, AppError> {
 
     let log_folder_string = check_essential_string (toml_folders.log_folder_path, "log folder", "log_folder_path")?;
     let json_files_string = check_essential_string (toml_folders.json_files_path, "json files folder", "json_files_path")?;
     let source_data_string = check_defaulted_string (toml_folders.source_data_path, "source data folder", "");

     Ok(FolderPars {
         log_folder_path: PathBuf::from(log_folder_string),
         json_files_path: PathBuf::from(json_files_string),
         source_data_path: PathBuf::from(source_data_string),
     })
 }
 
 
 fn verify_db_parameters(toml_database: TomlDBPars) -> Result<DBPars, AppError> {
 
     // Check user name and password first as there are no defaults for these values.
     // They must therefore be present.
 
     let db_user = check_essential_string (toml_database.db_user, "database user name", "db_user")?; 
 
     let db_password = check_essential_string (toml_database.db_password, "database user password", "db_password")?;
        
     let db_host = check_defaulted_string (toml_database.db_host, "DB host", "localhost");
             
     let db_port_as_string = check_defaulted_string (toml_database.db_port, "DB port", "5432");
     let db_port: usize = db_port_as_string.parse().unwrap_or_else(|_| 5432);
 
     let db1_name = check_defaulted_string (toml_database.db1_name, "DB1 name", "ctg1");
     let db2_name = check_defaulted_string (toml_database.db2_name, "DB2 name", "ctg2");
     let db3_name = check_defaulted_string (toml_database.db3_name, "DB3 name", "ctg3");
     let cxt_db_name = check_defaulted_string (toml_database.cxt_db_name, "Context DB name", "cxt");
     let mon_db_name = check_defaulted_string (toml_database.mon_db_name, "Monitor DB name", "mon");
 
     Ok(DBPars {
         db_host,
         db_user,
         db_password,
         db_port,
         db1_name,
         db2_name,
         db3_name,        
         cxt_db_name,
         mon_db_name,
     })
 }
 
 
 fn check_essential_string (src_name: Option<String>, value_name: &str, config_name: &str) -> Result<String, AppError> {
  
     let s = match src_name {
         Some(s) => s,
         None => "none".to_string(),
     };
 
     if s == "none".to_string() || s.trim() == "".to_string()
     {
         return Result::Err(AppError::ConfigurationError("Essential configuration value missing or misspelt.".to_string(),
         format!("Cannot find a value for {} ({}).", value_name, config_name)))
     }
     else {
         Ok(s)
     }
 }
 
 
 fn check_defaulted_string (src_name: Option<String>, value_name: &str, default:  &str) -> String {
  
     let s = match src_name {
         Some(s) => s,
         None => "none".to_string(),
     };
 
     if s == "none".to_string() || s.trim() == "".to_string()
     {
         println!("No value found for {} path in config file - 
         using the provided default value ('{}') instead.", value_name, default);
         default.to_string()
     }
     else {
        s
     }
 }
  

 pub fn fetch_db_name(db: &str) -> Result<String, AppError> {
     let db_pars = match DB_PARS.get() {
          Some(dbp) => dbp,
          None => {
             return Result::Err(AppError::MissingDBParameters());
         },
     };

     let db_name = match db {
        "db1" => db_pars.db1_name.clone(), 
        "db2" => db_pars.db2_name.clone(),
        "db3" => db_pars.db3_name.clone(), 
        "cxt" => db_pars.cxt_db_name.clone(), 
        "mon" => db_pars.mon_db_name.clone(), 
        _ => "".to_string(), 
     };
     Ok(db_name)
 }
 
 
 pub fn fetch_db_conn_string(db_name: &String) -> Result<String, AppError> {
     let db_pars = match DB_PARS.get() {
          Some(dbp) => dbp,
          None => {
             return Result::Err(AppError::MissingDBParameters());
         },
     };
     
     Ok(format!("postgres://{}:{}@{}:{}/{}", 
     db_pars.db_user, db_pars.db_password, db_pars.db_host, db_pars.db_port, db_name))
 }
 
 
 
 #[cfg(test)]
 mod tests {
     use super::*;
     
     // Ensure the parameters are being correctly extracted from the config file string
     
     #[test]
     fn check_config_with_all_params_present() {
 
         let config = r#"
[folders]
log_folder_path="/home/steve/Data/MDR logs/ctg/"
json_files_path="/home/steve/Data/MDR json files/ctg/"
source_data_path="/home/steve/Data/MDR source data/CTGDumps/20260410/"

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
         let res = populate_config_vars(&config_string).unwrap();

         assert_eq!(res.folders.log_folder_path, PathBuf::from("/home/steve/Data/MDR logs/ctg/"));
         assert_eq!(res.folders.json_files_path, PathBuf::from("/home/steve/Data/MDR json files/ctg/"));
         assert_eq!(res.folders.source_data_path, PathBuf::from("/home/steve/Data/MDR source data/CTGDumps/20260410/"));

         assert_eq!(res.db_pars.db_host, "localhost");
         assert_eq!(res.db_pars.db_user, "user_name");
         assert_eq!(res.db_pars.db_password, "password");
         assert_eq!(res.db_pars.db_port, 5432);
         assert_eq!(res.db_pars.db1_name, "ctg1");
         assert_eq!(res.db_pars.db2_name, "ctg2");
         assert_eq!(res.db_pars.db3_name, "ctg3");
         assert_eq!(res.db_pars.cxt_db_name, "cxt");
         assert_eq!(res.db_pars.mon_db_name, "mon");
    }
     


    #[test]
    #[should_panic]
     fn check_config_with_blank_log_folder() {
 
         let config = r#"
[folders]
log_folder_path=""
json_files_path="/home/steve/Data/MDR json files/ctg/"
source_data_path="/home/steve/Data/MDR source data/CTGDumps/20260410/"

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
         let _res = populate_config_vars(&config_string).unwrap();
         
 }


    #[test]
    #[should_panic]
    fn check_config_with_blank_files_folder() {
 
         let config = r#"
[folders]
log_folder_path="/home/steve/Data/MDR logs/ctg/"
json_files_path=""
source_data_path="/home/steve/Data/MDR source data/CTGDumps/20260410/"

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
         let _res = populate_config_vars(&config_string).unwrap();
         
 }
   
 
     #[test]
     #[should_panic]
     fn check_missing_user_name_panics() {
 
          let config = r#"
[folders]
log_folder_path="/home/steve/Data/MDR logs/ctg/"
json_files_path="/home/steve/Data/MDR json files/ctg/"
source_data_path="/home/steve/Data/MDR source data/CTGDumps/20260410/"

[database]
db_host="localhost"
db_user=""
db_password="password"
db_port="5432"

db1_name="ctg1"
db2_name="ctg2"
db3_name="ctg3"
mon_db_name="mon"
cxt_db_name="cxt"

 "#;
         let config_string = config.to_string();
         let _res = populate_config_vars(&config_string).unwrap();
     }
 
 
     #[test]
     fn check_db_defaults_are_supplied() {
 
          let config = r#"
[folders]
log_folder_path="/home/steve/Data/MDR logs/ctg/"
json_files_path="/home/steve/Data/MDR json files/ctg/"
source_data_path="/home/steve/Data/MDR source data/CTGDumps/20260410/"

[database]
db_host=""
db_user="user_name"
db_password="password"
db_port=""

db1_name=""
db2_name=""
db3_name=""
mon_db_name=""
cxt_db_name=""
 "#;
         let config_string = config.to_string();
         let res = populate_config_vars(&config_string).unwrap();
         assert_eq!(res.db_pars.db_host, "localhost");
         assert_eq!(res.db_pars.db_user, "user_name");
         assert_eq!(res.db_pars.db_password, "password");
         assert_eq!(res.db_pars.db_port, 5432);
         assert_eq!(res.db_pars.db1_name, "ctg1");
         assert_eq!(res.db_pars.db2_name, "ctg2");
         assert_eq!(res.db_pars.db3_name, "ctg3");
         assert_eq!(res.db_pars.cxt_db_name, "cxt");
         assert_eq!(res.db_pars.mon_db_name, "mon");

     }
 
 
 #[test]
     fn check_missing_port_gets_default() {
 
          let config = r#"
[folders]
log_folder_path="/home/steve/Data/MDR logs/ctg/"
json_files_path="/home/steve/Data/MDR json files/ctg/"
source_data_path="/home/steve/Data/MDR source data/CTGDumps/20260410/"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port=""

db1_name="ctg1"
db2_name="ctg2"
db3_name="ctg3"
mon_db_name="mon"
cxt_db_name="cxt"
 "#;
         let config_string = config.to_string();
         let res = populate_config_vars(&config_string).unwrap();
 
         assert_eq!(res.db_pars.db_host, "localhost");
         assert_eq!(res.db_pars.db_user, "user_name");
         assert_eq!(res.db_pars.db_password, "password");
         assert_eq!(res.db_pars.db_port, 5432);

     }
 
 }
   
 
 
