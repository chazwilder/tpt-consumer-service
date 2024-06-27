use sqlx_oldapi::mssql::{MssqlPoolOptions, Mssql};
use sqlx_oldapi::Pool;
use dotenvy::dotenv;
use std::env;


pub async fn get_connection() -> Result<Pool<Mssql>, anyhow::Error> {
    dotenv().ok();
    let pool = MssqlPoolOptions::new()
        .max_connections(20)
        .connect(env::var("MSSQL_URL").unwrap_or_default().as_str())
        .await
        .expect("Failed to connect to database");
    Ok(pool)
}