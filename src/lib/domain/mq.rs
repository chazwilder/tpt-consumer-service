use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties, ExchangeKind, BasicProperties, Channel};
use mongodb::{change_stream::event::{ChangeStreamEvent, OperationType}, bson::Document};
use crate::domain::pre_check_snapshot::process_new_order;
use serde_json;
use dotenvy::dotenv;
use std::env;
use futures::StreamExt;
use log::{error, info};

pub async fn get_mq() -> Result<Channel, Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set");
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    info!("Connected to RabbitMQ, exchange declared: new_order");
    Ok(channel)
}

pub async fn new_order_listener() -> Result<(), Box<dyn std::error::Error>> {
    let mut consumer = get_mq().await?
    .basic_consume(
        "pre-check-snapshot",
        "pre-check-snapshot-consumer-rust",
        BasicConsumeOptions::default(),
        FieldTable::default(),
    )
    .await?;
    println!("Consumer started. Waiting for messages on pre-check-snapshot queue...");

    while let Some(delivery) = consumer.next().await {
        match delivery {
            Ok(delivery) => {
                println!("Received message: {:?}", delivery);
                process_new_order(delivery).await;
            }
            Err(e) => error!("Error in consumer: {:?}", e),
        }
    }
    Ok(())
}