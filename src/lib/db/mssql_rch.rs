use sqlx_oldapi::mssql::{MssqlPoolOptions, Mssql};
use sqlx_oldapi::Pool;
use dotenvy::dotenv;
use std::env;
use log::{error, info};

pub async fn get_connection() -> Option<Pool<Mssql>> {
    dotenv().ok();
    let db_url = match env::var("MSSQL_URL") {
        Ok(url) => url,
        Err(e) => {
            error!("Failed to get MSSQL_URL from environment: {}", e);
            return None;
        }
    };

    match MssqlPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            info!("Successfully connected to database");
            Some(pool)
        }
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            None
        }
    }
}