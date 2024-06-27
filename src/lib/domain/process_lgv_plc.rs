use lapin::message::Delivery;
use lapin::options::{BasicAckOptions};
use serde_json::{Value};
use crate::models::iplc::ILgv;
use crate::domain::lgv_plc_to_mssql;

pub async fn process_lgv_plc(delivery: Delivery) -> Result<(), Box<dyn std::error::Error>> {
    let message = String::from_utf8(delivery.data.clone())?;
    let v: Value = serde_json::from_str(&message)?;
    let lgv: ILgv = serde_json::from_value(v)?;
    lgv_plc_to_mssql(lgv).await;

    delivery.ack(BasicAckOptions::default()).await?;
    Ok(())
}