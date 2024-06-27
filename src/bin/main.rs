use log4rs;
use tpt_consumer::domain::mq::{lgv_plc_listener, new_order_listener};
use tokio;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("C:\\Users\\cwilder\\Desktop\\dev\\TPT\\consumer_service\\log4rs.yaml", Default::default())
    .expect("Failed to initialize logger from config file");
    let health_route = warp::path("heartbeat").map(|| "OK");
    let server = tokio::spawn(warp::serve(health_route).run(([0, 0, 0, 0], 3033)));

    tokio::select! {
        result = new_order_listener() => {
            if let Err(e) = result {
                eprintln!("Error in new_order_listener: {:?}", e);
                std::process::exit(1);
            }
        },
        result = lgv_plc_listener() => {
            if let Err(e) = result {
                eprintln!("Error in lgv_plc_listener: {:?}", e);
                std::process::exit(1);
            }
        },
        _ = tokio::signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down");
        },
    }

    server.abort();
    println!("Shutting down");
    Ok(())
}
