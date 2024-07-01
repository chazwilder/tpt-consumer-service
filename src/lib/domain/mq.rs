use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties, Channel};
use crate::domain::pre_check_snapshot::process_new_order;
use crate::domain::process_lgv_plc;
use dotenvy::dotenv;
use std::env;
use futures::StreamExt;
use log::{error, info};
use tokio::sync::broadcast::Receiver;

pub async fn get_mq() -> Result<Channel, Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set");
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;
    channel.basic_qos(10, BasicQosOptions::default()).await.unwrap();

    Ok(channel)
}

pub async fn new_order_listener(x: &mut Receiver<()>) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set");
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;
    channel.basic_qos(10, BasicQosOptions::default()).await.unwrap();

    let mut consumer = channel
    .basic_consume(
        "pre-check-snapshot",
        "pre-check-snapshot-consumer-rust",
        BasicConsumeOptions::default(),
        FieldTable::default(),
    )
    .await?;
    info!("Connected to RabbitMQ, exchange declared: new_order");
    println!("Consumer started. Waiting for messages on pre-check-snapshot queue...");

    while let Some(delivery) = consumer.next().await {
        match delivery {
            Ok(delivery) => {
                info!("New Order Received message: {:?}", delivery);
                let _ = process_new_order(delivery).await;
            }
            Err(e) => error!("Error in consumer: {:?}", e),
        }
    }
    Ok(())
}


pub async fn lgv_plc_listener(x: &mut Receiver<()>) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set");
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;
    channel.basic_qos(10, BasicQosOptions::default()).await.unwrap();

    let mut consumer = channel
    .basic_consume(
        "lgv_plc_log",
        "rust",
        BasicConsumeOptions::default(),
        FieldTable::default(),
    )
    .await?;
    info!("Connected to RabbitMQ, exchange declared: lgv_plc");
    println!("Consumer started. Waiting for messages on LGV PLC queue...");

    while let Some(delivery) = consumer.next().await {
        match delivery {
            Ok(delivery) => {
                println!("PLC Received message: {:?}", delivery);
                let _ = process_lgv_plc(delivery).await;
            }
            Err(e) => error!("Error in consumer: {:?}", e),
        }
    }
    Ok(())
}