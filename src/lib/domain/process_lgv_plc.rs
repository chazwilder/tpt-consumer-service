use lapin::message::Delivery;
use lapin::options::{BasicAckOptions};
use log::{error, info};
use serde_json::{Value};
use crate::models::iplc::ILgv;
use crate::domain::lgv_plc_to_mssql;

pub async fn process_lgv_plc(delivery: Delivery) -> Result<(), Box<dyn std::error::Error>> {
    let message = String::from_utf8(delivery.data.clone())?;
    let v: Value = serde_json::from_str(&message)?;
    let lgv: ILgv = serde_json::from_value(v)?;
    match lgv_plc_to_mssql(lgv).await {
        Ok(_) => {
            delivery.ack(BasicAckOptions::default()).await?;
            info!("Message processed and acknowledged successfully");
        }
        Err(e) => {
            error!("Failed to process message: {}", e);
        }
    }
    Ok(())
}