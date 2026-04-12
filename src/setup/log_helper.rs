/***************************************************************************
 * Establishes the log for the programme's operation using log and log4rs, 
 * and includes various helper functions.
 ***************************************************************************/

 use chrono::Local;
 use std::path::PathBuf;
 use crate::err::AppError;
 use crate::setup::InitParams;
 
 use log::{info, LevelFilter};
 use log4rs::{
     append::{
         console::{ConsoleAppender, Target},
         file::FileAppender,
     },
     config::{Appender, Config, Root},
     encode::pattern::PatternEncoder,
 };
 
 pub fn setup_log (data_folder: &PathBuf) -> Result<log4rs::Handle, AppError> {
     let datetime_string = Local::now().format("%m-%d %H%M%S").to_string();
     let log_file_name = format!("CGT DB processing at {}.log", datetime_string);
     let log_file_path = [data_folder, &PathBuf::from(log_file_name)].iter().collect();
     config_log (&log_file_path)
 }
 
 
 fn config_log (log_file_path: &PathBuf) -> Result<log4rs::Handle, AppError> {
     
     // Initially establish a pattern for each log line.
 
     let log_pattern = "{d(%d/%m %H:%M:%S)}  {h({l})}  {({M}.{L}):>38.48}:  {m}\n";
 
     // Define a stderr logger, as one of the 'logging' sinks or 'appender's.
 
     let stderr = ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new(log_pattern)))
         .target(Target::Stderr).build();
 
     // Define a second logging sink or 'appender' - to a log file (provided path will place it in the current log folder).
 
     let logfile = FileAppender::builder().encoder(Box::new(PatternEncoder::new(log_pattern)))
         .build(log_file_path)
         .map_err(|e| AppError::IoWriteErrorWithPath(e, log_file_path.to_owned()))?;
 
     // Configure and build log4rs instance, using the two appenders described above
 
     let config = Config::builder()
         .appender(Appender::builder()
                 .build("logfile", Box::new(logfile)),)
         .appender(Appender::builder()
                 .build("stderr", Box::new(stderr)),)
         .build(Root::builder()
                 .appender("logfile")
                 .appender("stderr")
                 .build(LevelFilter::Info),
         )
         .map_err(|e| AppError::LogSetupError("Error when creating log4rs configuration".to_string(), e.to_string()))?;
 
     log4rs::init_config(config)
         .map_err(|e| AppError::LogSetupError("Error when creating log4rs handle".to_string(), e.to_string()))
 
 }
 
 
 pub fn log_startup_params (ip : &InitParams) {
     
     // Called at the end of set up to record the input parameters
 
     info!("PROGRAM START");
     info!("");
     info!("************************************");
     info!("");
     info!("log folder: {}", ip.log_folder_path.display());
     info!("json files folder: {}", ip.json_files_path.display());
     info!("download recent: {}", ip.flags.download_recent);
     info!("download set: {}", ip.flags.download_set);
     info!("download year: {}", ip.flags.download_year);
     info!("process recent: {}", ip.flags.process_recent);
     info!("process set: {}", ip.flags.process_set);
     info!("code uncoded {}", ip.flags.code_uncoded);
     info!("code all {}", ip.flags.code_all);
     info!("test run: {}", ip.flags.is_test);
     info!("start date {}", match &ip.start_date {Some(s) => s, None => "None"});
     info!("end date: {}", match &ip.end_date {Some(s) => s, None => "None"});
     info!("");
     info!("************************************");
     info!("");
 }