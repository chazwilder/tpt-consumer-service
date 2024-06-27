use crate::db::mssql_rch::get_connection;
use crate::models::ishipment_details::ILoadDetails;
use sqlx_oldapi;
use std::env;
use dotenvy::dotenv;
use log::{info, error};


pub async fn invenotry_snapshot(trip_number: &i64) -> Result<Vec<ILoadDetails>, Box<dyn std::error::Error>> {
    dotenv().ok();
    let pool = get_connection().await?;
    let sp = env::var("INVENTORY_QUERY").unwrap_or_default();
    let sql = format!("{} {}", sp, trip_number);
    match sqlx_oldapi::query_as::<_, ILoadDetails>(&sql).fetch_all(&pool).await {
        Ok(rows) => {
            info!("Retrieved {} new orders from SQL Server", rows.len());
            println!("{:?}", &rows);
            Ok(rows)
        }
        Err(err) => {
            error!("Error retrieving new orders from SQL Server: {:?}", err);
            Err(err.into())
        }
    }
}