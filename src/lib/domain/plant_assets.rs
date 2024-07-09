use log::{error, info};
use crate::db::get_connection;
use dotenvy::dotenv;
use std::env;
use chrono::NaiveDateTime;
use sqlx_oldapi::FromRow;
use serde::{Deserialize, Serialize};
use derive_more::{Constructor,Into};
use lapin::message::Delivery;
use lapin::options::BasicAckOptions;
use serde_json::Value;
use crate::db::mongodb_rch::save_assets;
use crate::models::imq_new_order::INewOrder;

#[derive(Debug, Clone, Serialize, Deserialize, Constructor, Into, FromRow)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct IPlantAssets {
    #[sqlx(rename = "LOG_DTTM")]
    pub log_dttm: NaiveDateTime,
    #[sqlx(rename = "DOCKS_ENABLED")]
    pub docks_enabled: i32,
    #[sqlx(rename = "DOCKS_DISABLED")]
    pub docks_disabled: i32,
    #[sqlx(rename = "TOTAL_DOCKS")]
    pub total_docks: i32,
    #[sqlx(rename = "PRELOAD")]
    pub preload: i32,
    #[sqlx(rename = "LIVE_LOAD")]
    pub live_load: i32,
    #[sqlx(rename = "LOAD_COUNT")]
    pub load_count: i32,
    #[sqlx(rename = "DOORS_AVAILABLE")]
    pub doors_available: i32,
    #[sqlx(rename = "REMOVED_LGVS")]
    pub removed_lgvs: i32,
    #[sqlx(rename = "FLEET_COUNT")]
    pub fleet_count: i32,

}

#[derive(Debug, Clone, Serialize, Deserialize, Constructor, Into, FromRow)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MPlantAsset {
    pub log_dttm: String,
    pub docks_enabled: i32,
    pub docks_disabled: i32,
    pub total_docks: i32,
    pub preload: i32,
    pub live_load: i32,
    pub load_count: i32,
    pub doors_available: i32,
    pub removed_lgvs: i32,
    pub fleet_count: i32,
}



pub async fn update_assets() -> Result<IPlantAssets, anyhow::Error> {
    dotenv().ok();
    let pool = match get_connection().await {
        Some(pool) => pool,
        None => {
            error!("No Connection To Database");
            return Err(anyhow::anyhow!("No Connection To Database"));
        }
    };
    let sp = env::var("ASSESTS_QUERY").unwrap_or_default();
    let sql = format!("{}", sp);
    info!("{}", &sql);
    match sqlx_oldapi::query_as::<_, IPlantAssets>(&sql).fetch_one(&pool).await {
        Ok(rows) => {
            info!("Retrieved {:?} new orders from SQL Server", &rows);
            println!("{:?}", &rows);
            Ok(rows)
        }
        Err(err) => {
            error!("Error retrieving new orders from SQL Server: {:?}", err);
            Err(err.into())
        }
    }
}

pub async fn process_plant_assets(delivery: Delivery) -> Result<(), Box<dyn std::error::Error>> {
    let message = String::from_utf8(delivery.data.clone())?;
    let v: Value = serde_json::from_str(&message)?;
    info!("{}", &message);
    let trip_number: i32 = v["TRIP_NUMBER"].as_i64().unwrap() as i32;
    let new_order = MPlantAsset {
        log_dttm: v["LOG_DTTM"].to_string(),
        docks_enabled: v["DOCKS_ENABLED"].as_i64().unwrap() as i32,
        docks_disabled: v["DOCKS_DISABLED"].as_i64().unwrap() as i32,
        total_docks: v["TOTAL_DOCKS"].as_i64().unwrap() as i32,
        preload: v["PRELOAD"].as_i64().unwrap() as i32,
        live_load: v["LIVE_LOAD"].as_i64().unwrap() as i32,
        load_count: v["LOAD_COUNT"].as_i64().unwrap() as i32,
        doors_available: v["DOORS_AVAILABLE"].as_i64().unwrap() as i32,
        removed_lgvs: v["REMOVED_LGVS"].as_i64().unwrap() as i32,
        fleet_count: v["FLEET_COUNT"].as_i64().unwrap() as i32,
    };
    info!("New Asset: {:?}", &new_order);
    let _ = save_assets(new_order, trip_number).await;
    delivery.ack(BasicAckOptions::default()).await?;

    Ok(())
}
