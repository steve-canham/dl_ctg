pub mod base_types;
pub mod err;
pub mod setup;
pub mod iec;
pub mod encode;
pub mod download;
pub mod import;
pub mod data_models;

use download::monitoring::{get_next_download_id, update_dl_event_record};
use crate::base_types::{DownloadResult, DownloadType, ImportType, EncodingType};
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

    setup::establish_log(&params)?;

    let mon_pool = setup::get_db_pool("mon").await?;     

    if params.download_type != DownloadType::None {

        let dl_id = get_next_download_id(100120, &params.download_type, &mon_pool).await?;
        let mut dl_res = DownloadResult::new();
        if params.download_type == DownloadType::Recent {
        
        }

        if params.download_type == DownloadType::BetweenDates{
        
        }

        if params.download_type == DownloadType::AllFromFolders {

            dl_res = download::do_folder_download(dl_id, &params).await?;
            
        }
        update_dl_event_record (dl_id, dl_res, &params, &mon_pool).await?;
        
    
    }


    if params.import_type != ImportType::None {

        if params.import_type == ImportType::Recent {
        
        }

        if params.import_type == ImportType::BetweenDates {
        
        }
    }


    if params.encoding_type != EncodingType::None {

         if params.encoding_type == EncodingType::Recent {
        
        }

        if params.encoding_type == EncodingType::All {
        
        }

    }

    Ok(())  

}



