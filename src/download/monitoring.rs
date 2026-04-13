use crate::base_types::*;
use crate::AppError;

use sqlx::{Pool, Postgres};
use std::path::PathBuf;
use chrono::Utc;


pub async fn update_ctg_mon(sd_sid: &String, remote_url: &String, dl_id: i32,
                     record_date: &Option<String>, full_path: &PathBuf, src_pool: &Pool<Postgres>) -> Result<bool, AppError> {

        let mut added = false;          // indicates if will be a new record or update of an existing one
        let now = Utc::now();
        let local_path = full_path.to_str().unwrap();  // assumes utf-8 characters
        
        let sql = format!("SELECT EXISTS(SELECT 1 from mn.source_data where sd_sid = '{}')", sd_sid); 
        let mon_record_exists = sqlx::query_scalar(&sql).fetch_one(src_pool).await
                        .map_err(|e| AppError::SqlxError(e, sql))?;

        if mon_record_exists {   // Row already exists - update with new details.
            
            let sql = r#"Update mn.source_data set 
                        remote_url = $2,
                        last_revised = $3::timestamp,
                        local_path = $4,
                        last_dl_id = $5,
                        last_downloaded = $6
                        where sd_sid = $1;"#;
            sqlx::query(&sql).bind(sd_sid).bind(remote_url).bind(record_date)    
            .bind(local_path).bind(dl_id).bind(now).execute(src_pool).await
                    .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;       
        }
        else {   // Create as a new record.
            
            let sql = r#"Insert into mn.source_data(sd_sid, remote_url, last_revised,
	                    local_path, last_dl_id, last_downloaded) values ($1, $2, $3::timestamp, $4, $5, $6)"#;
            sqlx::query(&sql).bind(sd_sid).bind(remote_url).bind(record_date)    
            .bind(local_path).bind(dl_id).bind(now).execute(src_pool).await
                    .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;     
            added = true;  
        }

        Ok(added)
}


pub async fn get_next_download_id(source_id: i32, dl_type: &DownloadType, mon_pool: &Pool<Postgres>) -> Result<i32, AppError>{

    let sql = "select coalesce(max(id), 10001) from evs.dl_events ";
    let last_id: i32 = sqlx::query_scalar(sql).fetch_one(mon_pool)
                      .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    let new_id = last_id + 1;
    
    // Create the new record (to be updated later).

    let now = Utc::now();
    let sql = "Insert into evs.dl_events(id, source_id, dl_type, time_started) values ($1, $2, $3, $4)";
    sqlx::query(sql).bind(new_id).bind(source_id).bind(dl_type.to_string()).bind(now)
            .execute(mon_pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(new_id)
}


pub async fn update_dl_event_record (dl_id: i32, dl_res: DownloadResult, params: &InitParams, mon_pool: &Pool<Postgres>) ->  Result<bool, AppError> {
     
    let now = Utc::now();
    let sql = r#"Update evs.dl_events set 
             time_ended = $2,
             num_records_checked = $3,
             num_records_downloaded = $4,
             num_records_added = $5,
             start_date = $6,
             end_date = $7,
             filefolder_path = $8
             where id = $1"#;
    let res = sqlx::query(sql).bind(dl_id).bind(now)
            .bind(dl_res.num_checked).bind(dl_res.num_downloaded).bind(dl_res.num_added)
            .bind(params.start_date.clone()).bind(params.end_date.clone()).bind(params.json_files_path.to_string_lossy())
            .execute(mon_pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?; 
    Ok(res.rows_affected() == 1)
}

