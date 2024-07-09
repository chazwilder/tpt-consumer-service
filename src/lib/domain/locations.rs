use serde::{Deserialize, Serialize};
use derive_more::{Constructor, Into};
use sqlx_oldapi::FromRow;
use chrono::NaiveDateTime;
use log::{error, info};
use crate::db::get_connection;
use dotenvy::dotenv;
use std::env;
use lapin::message::Delivery;
use lapin::options::BasicAckOptions;
use serde_json::Value;
use crate::db::mongodb_rch::save_locations;
use crate::domain::plant_assets::IPlantAssets;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Constructor, FromRow, Into)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ISkuLocation {
    pub id: i64,
    pub id_storage_id: i64,
    pub storage_location: String,
    pub sku: String,
    pub description: String,
    pub quantity: f64,
    pub reserved_units: i64,
    pub total_units: i64,
    pub available_units: i64,
    pub blocked_units: i64,
    pub location_retrieval: bool,
    pub location_storage: bool,
    pub id_position: i64,
    pub storage_position: i64,
    pub position_level: i64,
    pub position_row: i64,
    pub retrieval_red_block: bool,
    pub retrieval_disabled: bool,
    pub storage_red_block: bool,
    pub storage_disabled: bool,
    pub id_stock_unit: i64,
    pub lpn: String,
    pub enter_date_time: NaiveDateTime,
    pub aging_lpns: i64,
    pub original_item_maturity_days: i64,
    pub original_days_to_expiry: i64,
    pub auto_hold_hours: i64,
    pub ship_by_date_time: Option<NaiveDateTime>,
    pub lot_number: Option<String>,
    pub load_aid: String,
    pub quality_status: String,
    pub validation_update_dttm: NaiveDateTime,
    pub validation_update_user: String,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub weight: f64,
    pub default_shelf_life: i64,
    pub days_to_expiry: i64,
    pub id_sales_velocity: i64,
    pub store_equal_age: i64,
    pub dps_pick_equal_age: i64,
    pub sps_pick_equal_age: i64,
    pub sps_dps_pick_equal_age: i64,
    pub stability: i64,
    pub days_to_expiration_preventing_x_doc: Option<i64>,
    pub days_to_expiration_forcing_fefo_pick: i64,
}


pub async fn update_locations(sku: &str) -> Result<Vec<ISkuLocation>, anyhow::Error> {
    dotenv().ok();
    let pool = match get_connection().await {
        Some(pool) => pool,
        None => {
            error!("No Connection To Database");
            return Err(anyhow::anyhow!("No Connection To Database"));
        }
    };
    let sp = env::var("LOCATIONS_QUERY").expect("Missing LOCATIONS_QUERY environment variable");
    let sql = sp.replace("{}", &sku);
    info!("{}", &sql);
    match sqlx_oldapi::query_as::<_, ISkuLocation>(&sql).fetch_all(&pool).await {
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

pub async fn process_locations(delivery: Delivery) -> Result<(), Box<dyn std::error::Error>> {
    let message = String::from_utf8(delivery.data.clone())?;
    let v: Value = serde_json::from_str(&message)?;
    let trip_number = v["TRIP_NUMBER"].as_i64().unwrap_or_default() as i32;
    let loco = v["LOCATIONS"].clone();
    let locations: Vec<ISkuLocation> = serde_json::from_value(loco)?;
    match save_locations(locations,trip_number).await {
        Ok(rows) => {
            delivery.ack(BasicAckOptions::default()).await?;
            info!("Message processed and acknowledged successfully");
        }
        Err(e) => {
            error!("Error processing message: {:?}", e);
        }
    }
    Ok(())
}