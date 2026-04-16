use std::ops::Add;
use chrono::NaiveDate;
use std::path::PathBuf;


pub struct InitParams {
    pub log_folder_path: PathBuf,
    pub json_files_path: PathBuf,
    pub source_data_path: PathBuf,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub download_type: DownloadType,
    pub import_type: ImportType,
    pub encoding_type: EncodingType,
    pub is_test: bool,
}


#[derive(PartialEq, Debug)]
pub enum DownloadType {
    Recent,
    BetweenDates,
    AllFromFolders,
    None
}

impl DownloadType {
    pub fn to_string(&self) -> String {
        match self { 
            DownloadType::Recent => "Data recently updated".to_string(), 
            DownloadType::BetweenDates => "Data from studies created in specified months".to_string(), 
            DownloadType::AllFromFolders => "All Data,from raw API data folders".to_string(), 
            DownloadType::None => "None".to_string(), 
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ImportType {
    None,
    Recent,
    BetweenDates,
}

impl ImportType {
    pub fn to_string(&self) -> String {
        match self { 
            ImportType::None => "None".to_string(), 
            ImportType::Recent => "Data recently downloaded".to_string(), 
            ImportType::BetweenDates => "Data from studies created in specified months".to_string(), 
        }
    }
}


#[derive(PartialEq, Debug)]
pub enum EncodingType {
    None,
    Recent,
    All,
}

impl EncodingType {
    pub fn to_string(&self) -> String {
        match self { 
            EncodingType::None => "None".to_string(), 
            EncodingType::Recent => "Data currently uncoded".to_string(), 
            EncodingType::All => "All data".to_string(), 
        }
    }
}

#[derive(Clone)]
pub struct DownloadResult {
    pub num_checked: i32,
    pub num_downloaded: i32,
    pub num_added: i32,
}

impl DownloadResult {
    pub fn new() -> Self {
        DownloadResult {  
        num_checked: 0,
        num_downloaded: 0,
        num_added: 0,
        }
   }
}

impl Add for DownloadResult {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self{  
            num_checked: self.num_checked + other.num_checked,
            num_downloaded: self.num_downloaded + other.num_downloaded,
            num_added: self.num_added + other.num_added,
        }
    }
}

pub struct ImportResult {
    pub num_available: i64,
    pub num_imported: i64,
    pub earliest_dl_date: NaiveDate,
    pub latest_dl_date: NaiveDate,
}
