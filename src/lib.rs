
pub mod setup;
pub mod iec;
pub mod encode;
pub mod download;
pub mod import;
pub mod err;

use setup::cli_reader;
use err::AppError;
use std::ffi::OsString;
use std::path::PathBuf;
use std::fs;

pub async fn run(args: Vec<OsString>) -> Result<(), AppError> {

    let cli_pars: cli_reader::CliPars;
    cli_pars = cli_reader::fetch_valid_arguments(args)?;
    
    let config_file = PathBuf::from("./app_config.toml");
    let config_string: String = fs::read_to_string(&config_file)
                                .map_err(|e| AppError::IoReadErrorWithPath(e, config_file))?;
                              
    let params = setup::get_params(cli_pars, &config_string)?;
    let flags = params.flags;
    setup::establish_log(&params)?;

    let _pool1 = setup::get_db_pool("db1").await?;
    let _pool2 = setup::get_db_pool("db2").await?;
    let _pool3 = setup::get_db_pool("db3").await?;
    let _cxt_pool = setup::get_db_pool("cxt").await?;
    let _mon_pool = setup::get_db_pool("mon").await?;     
          

    if flags.download_recent {
       //mdr::do_mdr_import(&params.data_date, &pool).await?;
    }


    if flags.download_set {
       //mdr::do_mdr_import(&params.data_date, &pool).await?;
    }


    if flags.download_year {
       //mdr::do_mdr_import(&params.data_date, &pool).await?;
    }



    //if flags.code_data {
        //encode::do_data_encoding(&pool).await?;
    //}



    Ok(())  
}



