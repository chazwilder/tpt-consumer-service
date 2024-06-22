use lapin::message::Delivery;
use lapin::options::BasicAckOptions;

pub async fn process_new_order(delivery: Delivery) -> Result<(), Box<dyn std::error::Error>> {
    let message = String::from_utf8(delivery.data.clone())?;
    println!("New order: {}", message);


    delivery.ack(BasicAckOptions::default()).await?;
    Ok(())
}