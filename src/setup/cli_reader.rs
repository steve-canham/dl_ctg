/*************************************************************************

CTG data can be downloaded in three ways:
a) Using the APi to obtain the study data updated on or after
a set cut-off date (flag r, string d as an ISO date).
b) Using the API to get data related to all studies first posted 
in a set of consecutive months (flag m, strings d and e as year-months).
c) By manually downloading data on a year-by-year basis, as json files
(flag y, string d as a four digit year).
In all cases a 'd' string is required, as a full or partiaL date in 
ISO format.   
In all cases the ingested data is simplified to to fit the MDR's 
own json structure, and stored in the relevant json file folders,
with one folder for each month-year.

Once in the json file format the data can be processed, through
sd and ad schema stages, and then encoded to form a final MDR dataset.
Processing can occur:
a) By processing only that data downloaded since the last process 
(flag p)
b) By processing data in selected json folders, which needs start
and end month-years (= folder names). (flag q, strings d and e as 
year-months)

Encoding can be just of noncoded (i.e. recently processed) data 
(c flag), or it can be of all processed data (f flag).

The a flag carries out r, p and c in that order, i.e. dowbnloads, 
processes and encodews any data edited after a cutoff date (string d).
There is no default action - one or more CLI parameters must be 
provided.

Note that because of the volume of CTG data three different databases
are used. The first ctga, covewrs studies first posted before 2016, the 
second, ctgb covers studies first posted between 2016 and 2022 
inclusively, while the third, ctgc, covers studies first posted from
2023 onwards.
***********************************************************************/
  
 use clap::{command, Arg, ArgMatches};
 use crate::err::AppError;
 use crate::base_types::{DownloadType, ImportType, EncodingType};
 use std::ffi::OsString;
 use chrono::{NaiveDate, Utc, Datelike};
 
 pub struct CliPars {
    pub download_type: DownloadType,
    pub import_type: ImportType,
    pub encoding_type: EncodingType,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub is_test: bool,
 }
 

 pub fn fetch_valid_arguments(args: Vec<OsString>) -> Result<CliPars, AppError>
 { 
    let parse_result = parse_args(args)?;

    // Date parameters have defult values of "", therefore safe to unwrap
  
    let start_date = parse_result.get_one::<String>("start_date").unwrap();
    let end_date = parse_result.get_one::<String>("end_date").unwrap();

    // Flag values are false if not present, true if present.
 
    let a_flag = parse_result.get_flag("a_flag");
    let mut r_flag = parse_result.get_flag("r_flag");
    let m_flag = parse_result.get_flag("m_flag");
    let y_flag = parse_result.get_flag("y_flag");
    let mut p_flag = parse_result.get_flag("p_flag");
    let mut q_flag = parse_result.get_flag("q_flag");
    let mut c_flag = parse_result.get_flag("c_flag");
    let f_flag = parse_result.get_flag("f_flag");
    let z_flag = parse_result.get_flag("z_flag");

    if a_flag == true {
       r_flag = true;
       p_flag = true;
       c_flag = true;
    }
       
    let mut download_type = DownloadType::None;
    let mut import_type = ImportType::None;
    let mut encoding_type = EncodingType::None;

    if r_flag || m_flag || y_flag {

        // The three options have different -d parameter types
        // Only one will work for each, so having more than one of r, m and y
        // will result in an error from the non -d supported flag(s).
        // Therefore only one can be used and no need to check for 
        // multiple download flags.

        if r_flag {
            match NaiveDate::parse_from_str(start_date, "%Y-%m-%d") {
                Ok(d) => d,
                Err(_) => return Err(AppError::ConfigurationError("Essential configuration value missing or misspelt.".to_string(),
                        format!("Cannot find a valid value for cut-off date (given as {}).", start_date))),
            };
            download_type = DownloadType::Recent;
        }

        if m_flag {
        
            if !check_year_month(start_date) {
                return Err(AppError::ConfigurationError("Essential configuration value missing or misspelt.".to_string(),
                format!("Cannot find a valid value for start year-month (given as {}).", start_date)));
            }

            if !check_year_month(end_date) {
                return Err(AppError::ConfigurationError("Essential configuration value missing or misspelt.".to_string(),
                format!("Cannot find a valid value for end year-month (given as {}).", end_date)));
            }
            download_type = DownloadType::BetweenDates;
        }

        if y_flag {
            match start_date.parse::<i32>() {
                Ok(d) => {
                    let this_year = Utc::now().year();
                    if d < 1999 || d > this_year {
                        return Err(AppError::ConfigurationError("Essential configuration value missing or misspelt.".to_string(),
                        format!("Cannot find a valid value for year (given as {}).", start_date)));
                    }
                },
                Err(_) => return Err(AppError::ConfigurationError("Essential configuration value missing or misspelt.".to_string(),
                        format!("Cannot find a valid value for year (given as {}).", start_date))),
            };
            download_type = DownloadType::ByYear;
        }

    }


    if p_flag || q_flag {

        if p_flag && q_flag {
            q_flag = false;  // if both do process recent
        }

        if p_flag {
             import_type = ImportType::Recent;
        }

        if q_flag {
        
            if !check_year_month(start_date) {
                return Err(AppError::ConfigurationError("Essential configuration value missing or misspelt.".to_string(),
                format!("Cannot find a valid value for start year-month (given as {}).", start_date)));
            }

            if !check_year_month(end_date) {
                return Err(AppError::ConfigurationError("Essential configuration value missing or misspelt.".to_string(),
                format!("Cannot find a valid value for end year-month (given as {}).", end_date)));
            }
            import_type = ImportType::BetweenDates;

        }

    }

    if c_flag || f_flag {

        if c_flag && f_flag {
            c_flag = false;  // if both do code all
        }

        if c_flag {
            encoding_type = EncodingType::Recent;
        }
        else {
            encoding_type = EncodingType::All;
        }
    }
 
    Ok(CliPars {
        download_type: download_type,
        import_type: import_type,
        encoding_type: encoding_type,
        start_date: if start_date.trim() == "" {None} else {Some(start_date.clone())},
        end_date: if end_date.trim() == "" {None} else {Some(end_date.clone())},
        is_test: z_flag,
    })
 
 }
 

 fn check_year_month(ym: &String) -> bool {

    let mut response = false;

    let string_parts: Vec<&str> = ym.split('-').collect();
    if string_parts.len() == 2 {
        let yr = string_parts[0];
        let mn = string_parts[1];

        let yr_ok = match yr.parse::<i32>() {
            Ok(y) => {
                let this_year = Utc::now().year();
                if y < 1999 || y > this_year {
                    false
                }
                else {
                    true
                }
            },
            Err(_) => false,
        };
        let mn_ok = match mn.parse::<i32>() {
            Ok(m) => {
                if m < 1 || m > 12 {
                    false
                }
                else {
                    true
                }
            },
            Err(_) => false,
        };
        if yr_ok && mn_ok {
            response = true;
        }
    }
    response
    
 }

 
 fn parse_args(args: Vec<OsString>) -> Result<ArgMatches, clap::Error> {
 
    command!() 
        .about("Processes data using CTG API, transferring it to the MDR ctg database")
        .arg(
            Arg::new("r_flag")
            .short('r')
            .long("recent")
            .required(false)
            .help("A flag signifying download data edited on or after the cut-off date")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("m_flag")
            .short('m')
            .long("month")
            .required(false)
            .help("A flag signifying download data edited first posted in a particular period")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("y_flag")
            .short('y')
            .long("year")
            .required(false)
            .help("A flag signifying download data from a folder with a year's raw ctg data")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("p_flag")
            .short('p')
            .long("procrecent")
            .required(false)
            .help("A flag signifying process all data downloaded since the last process operation")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("q_flag")
            .short('q')
            .long("procset")
            .required(false)
            .help("A flag indicating process all studies first posted in a defined period")
            .action(clap::ArgAction::SetTrue)
        )
                .arg(
            Arg::new("c_flag")
            .short('c')
            .long("coderecent")
            .required(false)
            .help("A flag signifying code all data downloaded since the last coding process")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("f_flag")
            .short('f')
            .long("codeall")
            .required(false)
            .help("A flag indicating signifying (re)code all data")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("a_flag")
            .short('a')
            .long("allrecent")
            .required(false)
            .help("A flag indicating that recent data should be downloaded, processed and coded")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(
           Arg::new("start_date")
           .short('d')
           .long("startdate")
           .required(false)
           .help("A date or partial date in ISO format, used with the r, m, y, a and q flags ")
           .default_value("")
        )
        .arg(
           Arg::new("end_date")
           .short('e')
           .long("enddate")
           .required(false)
           .help("A partial date (yyyy-mm) used as the end date with the m and q flags")
           .default_value("")
        )
        .arg(
            Arg::new("z_flag")
             .short('z')
             .long("test")
             .required(false)
             .help("A flag signifying that this is part of a test run - suppresses logs")
             .action(clap::ArgAction::SetTrue)
        )
    .try_get_matches_from(args)
 
 }
 
 
 #[cfg(test)]
 mod tests {
     use super::*;
     
     // Ensure the parameters are being correctly extracted from the CLI arguments
  
     #[test]
     fn check_cli_with_r_flag_and_date() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-r", "-d", "2024-11-23"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.start_date, Some("2024-11-23".to_string()));
         assert_eq!(res.end_date, None);
         assert_eq!(res.flags.download_recent, true);
         assert_eq!(res.flags.download_set, false);
         assert_eq!(res.flags.download_year, false);
         assert_eq!(res.flags.process_recent, false);
         assert_eq!(res.flags.process_set, false);   
         assert_eq!(res.flags.code_uncoded, false);
         assert_eq!(res.flags.code_all, false);           
         assert_eq!(res.flags.is_test, false);
     }


     #[test]
     #[should_panic]
     fn check_cli_with_r_flag_and_invalid_date() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-r", "-d", "2024-02-31"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let _res = fetch_valid_arguments(test_args).unwrap();
     }
 
     #[test]
     fn check_cli_with_a_flag_and_date() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-a", "-d", "2024-11-23"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.start_date, Some("2024-11-23".to_string()));
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
 
 
     #[test]
     fn check_cli_with_m_flag_and_dates() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-m", "-d", "2024-11", "-e", "2025-06"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.start_date, Some("2024-11".to_string()));
         assert_eq!(res.end_date, Some("2025-06".to_string()));
         assert_eq!(res.flags.download_recent, false);
         assert_eq!(res.flags.download_set, true);
         assert_eq!(res.flags.download_year, false);
         assert_eq!(res.flags.process_recent, false);
         assert_eq!(res.flags.process_set, false);   
         assert_eq!(res.flags.code_uncoded, false);
         assert_eq!(res.flags.code_all, false);           
         assert_eq!(res.flags.is_test, false);
     }

     #[test]
     #[should_panic]
     fn check_cli_with_m_flag_and_invalid_start_date() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-m", "-d", "1924-11", "-e", "2025-06"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let _res = fetch_valid_arguments(test_args).unwrap();
     }

     #[test]
     #[should_panic]
     fn check_cli_with_m_flag_and_invalid_endt_date() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-m", "-d", "2024-11", "-e", "2025-00"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let _res = fetch_valid_arguments(test_args).unwrap();
     }
       
    
     #[test]
     fn check_cli_with_y_flag_and_date() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-y", "-d", "2024"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.start_date, Some("2024".to_string()));
         assert_eq!(res.end_date, None);
         assert_eq!(res.flags.download_recent, false);
         assert_eq!(res.flags.download_set, false);
         assert_eq!(res.flags.download_year, true);
         assert_eq!(res.flags.process_recent, false);
         assert_eq!(res.flags.process_set, false);   
         assert_eq!(res.flags.code_uncoded, false);
         assert_eq!(res.flags.code_all, false);           
         assert_eq!(res.flags.is_test, false);
     }


     #[test]
     #[should_panic]
     fn check_cli_with_y_flag_and_invalid_date() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-y", "-d", "2034"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let _res = fetch_valid_arguments(test_args).unwrap();
     }


     #[test]
     fn check_cli_with_p_flag() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-p"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.start_date, None);
         assert_eq!(res.end_date, None);
         assert_eq!(res.flags.download_recent, false);
         assert_eq!(res.flags.download_set, false);
         assert_eq!(res.flags.download_year, false);
         assert_eq!(res.flags.process_recent, true);
         assert_eq!(res.flags.process_set, false);   
         assert_eq!(res.flags.code_uncoded, false);
         assert_eq!(res.flags.code_all, false);           
         assert_eq!(res.flags.is_test, false);
     }



     #[test]
     fn check_cli_with_q_flag_and_dates() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-q", "-d", "2024-11", "-e", "2025-06"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.start_date, Some("2024-11".to_string()));
         assert_eq!(res.end_date, Some("2025-06".to_string()));
         assert_eq!(res.flags.download_recent, false);
         assert_eq!(res.flags.download_set, false);
         assert_eq!(res.flags.download_year, false);
         assert_eq!(res.flags.process_recent, false);
         assert_eq!(res.flags.process_set, true);   
         assert_eq!(res.flags.code_uncoded, false);
         assert_eq!(res.flags.code_all, false);           
         assert_eq!(res.flags.is_test, false);
     }


     #[test]
     fn check_cli_with_c_flag() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-c"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.start_date, None);
         assert_eq!(res.end_date, None);
         assert_eq!(res.flags.download_recent, false);
         assert_eq!(res.flags.download_set, false);
         assert_eq!(res.flags.download_year, false);
         assert_eq!(res.flags.process_recent, false);
         assert_eq!(res.flags.process_set, false);   
         assert_eq!(res.flags.code_uncoded, true);
         assert_eq!(res.flags.code_all, false);           
         assert_eq!(res.flags.is_test, false);
     }


     #[test]
     fn check_cli_with_f_flag() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-f"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.start_date, None);
         assert_eq!(res.end_date, None);
         assert_eq!(res.flags.download_recent, false);
         assert_eq!(res.flags.download_set, false);
         assert_eq!(res.flags.download_year, false);
         assert_eq!(res.flags.process_recent, false);
         assert_eq!(res.flags.process_set, false);   
         assert_eq!(res.flags.code_uncoded, false);
         assert_eq!(res.flags.code_all, true);           
         assert_eq!(res.flags.is_test, false);
     }


     #[test]
     fn check_cli_with_c_and_f_flag() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-c", "-f"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.start_date, None);
         assert_eq!(res.end_date, None);
         assert_eq!(res.flags.download_recent, false);
         assert_eq!(res.flags.download_set, false);
         assert_eq!(res.flags.download_year, false);
         assert_eq!(res.flags.process_recent, false);
         assert_eq!(res.flags.process_set, false);   
         assert_eq!(res.flags.code_uncoded, false);
         assert_eq!(res.flags.code_all, true);           
         assert_eq!(res.flags.is_test, false);
     }


          #[test]
     fn check_cli_with_c_and_z_flag() {
         let target = "dummy target";
         let args : Vec<&str> = vec![target, "-c", "-z"];
         let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
 
         let res = fetch_valid_arguments(test_args).unwrap();
         assert_eq!(res.start_date, None);
         assert_eq!(res.end_date, None);
         assert_eq!(res.flags.download_recent, false);
         assert_eq!(res.flags.download_set, false);
         assert_eq!(res.flags.download_year, false);
         assert_eq!(res.flags.process_recent, false);
         assert_eq!(res.flags.process_set, false);   
         assert_eq!(res.flags.code_uncoded, true);
         assert_eq!(res.flags.code_all, false);           
         assert_eq!(res.flags.is_test, true);
     }



 }
 
 