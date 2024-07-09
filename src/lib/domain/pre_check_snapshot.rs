use lapin::message::Delivery;
use lapin::options::BasicAckOptions;
use log::info;
use serde_json::{json, Value};
use crate::models::imq_new_order::INewOrder;
use crate::domain::inventory::invenotry_snapshot;
use crate::db::mongodb_rch::update_shipment;
use crate::domain::locations::update_locations;
use crate::domain::mq::publish_to_rabbitmq;
use crate::domain::plant_assets::update_assets;

pub async fn process_new_order(delivery: Delivery) -> Result<(), Box<dyn std::error::Error>> {
    let message = String::from_utf8(delivery.data.clone())?;
    let v: Value = serde_json::from_str(&message)?;
    info!("{}", &message);
    let new_order = INewOrder {
        mongo_id: v["fullDocument"]["_id"]["$oid"].to_string().trim_matches('"').to_string(),
        trip_number: v["fullDocument"]["TRIP_NUMBER"].as_f64().unwrap_or_default() as i64,
    };
    info!("New order: {:?}", &new_order);
    let inv = invenotry_snapshot(&new_order.trip_number).await?;
    update_shipment(inv.clone(), &new_order).await;
    let sku = inv.first().unwrap().SKU.clone();
    let trip_number = new_order.trip_number;
    let mut assets = update_assets().await?;
    let mut assets_json: Value = serde_json::to_value(&assets)?;
    assets_json["TRIP_NUMBER"] = json!(new_order.trip_number);
    let assets_json_string = serde_json::to_string_pretty(&assets_json)?;
    publish_to_rabbitmq("plant_assets_log", &assets_json_string).await?;
    let location = update_locations(&sku).await?;
    let mut location_json: Value = serde_json::to_value(&location)?;
    let loco_json = json!({"TRIP_NUMBER": new_order.trip_number, "LOCATIONS": location_json});
    let location_json_string = serde_json::to_string_pretty(&loco_json)?;
    publish_to_rabbitmq("locations_log", &location_json_string).await?;
    delivery.ack(BasicAckOptions::default()).await?;
    Ok(())
}