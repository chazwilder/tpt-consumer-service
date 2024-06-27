use lapin::message::Delivery;
use lapin::options::BasicAckOptions;
use serde_json::{Value};
use crate::models::imq_new_order::INewOrder;
use crate::domain::inventory::invenotry_snapshot;

pub async fn process_new_order(delivery: Delivery) -> Result<(), Box<dyn std::error::Error>> {
    let message = String::from_utf8(delivery.data.clone())?;
    let v: Value = serde_json::from_str(&message)?;
    let new_order = INewOrder {
        mongo_id: v["fullDocument"]["_id"]["$oid"].to_string().trim_matches('"').to_string(),
        trip_number: v["fullDocument"]["TRIP_NUMBER"].as_f64().unwrap_or_default() as i64,
    };
    println!("New order: {:?}", &new_order);
    let inv = invenotry_snapshot(&new_order.trip_number).await?;



    delivery.ack(BasicAckOptions::default()).await?;
    Ok(())
}