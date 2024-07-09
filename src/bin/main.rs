use log::{error, info};
use tpt_consumer::domain::mq::{lgv_plc_listener, new_order_listener, plant_asset_listener};
use tokio;
use warp::Filter;
use thiserror::Error;
use std::time::Duration;
use anyhow;
use dotenvy::dotenv;
use std::env;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to initialize logger: {0}")]
    LoggerInitError(#[from] anyhow::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("New order listener error: {0}")]
    NewOrderListenerError(String),
    #[error("LGV PLC listener error: {0}")]
    LgvPlcListenerError(String),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    log4rs::init_file(
        env::var("LOG4RS_DIR").expect("LOG4RS DIR NOT DEFINED."),
        Default::default(),
    ).map_err(|e| AppError::LoggerInitError(e.into()))?;

    info!("Starting application");

    let health_route = warp::path("heartbeat").map(|| "OK");
    let server = tokio::spawn(warp::serve(health_route).run(([0, 0, 0, 0], 3043)));

    let (shutdown_sender, mut shutdown_receiver) = tokio::sync::broadcast::channel::<()>(1);
    let shutdown_sender_clone = shutdown_sender.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
        info!("Received Ctrl+C, initiating shutdown");
        let _ = shutdown_sender_clone.send(());
    });

    let mut new_order_shutdown_receiver = shutdown_sender.subscribe();
    let mut lgv_plc_shutdown_receiver = shutdown_sender.subscribe();
    let mut plant_asset_shutdown_receiver = shutdown_sender.subscribe();

    loop {
        tokio::select! {
            result = new_order_listener(&mut new_order_shutdown_receiver) => {
                match result {
                    Ok(_) => {
                        info!("New Order listener completed successfully");
                    },
                    Err(e) => {
                        error!("New Order listener error: {}. Restarting in 5 seconds...", e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            },
            // result = lgv_plc_listener(&mut lgv_plc_shutdown_receiver) => {
            //     match result {
            //         Ok(_) => info!("LGV PLC listener completed successfully"),
            //         Err(e) => {
            //             error!("LGV PLC listener error: {}. Restarting in 5 seconds...", e);
            //             tokio::time::sleep(Duration::from_secs(5)).await;
            //         }
            //     }
            // },
            result = plant_asset_listener(&mut plant_asset_shutdown_receiver) => {
                match result {
                    Ok(_) => info!("PLANT ASSET listener completed successfully"),
                    Err(e) => {
                        error!("PLANT ASSET listener error: {}. Restarting in 5 seconds...", e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            },
            _ = shutdown_receiver.recv() => {
                info!("Shutdown signal received, stopping listeners");
                break;
            }
        }
    }

    // Graceful shutdown
    info!("Shutting down server");
    server.abort();
    let _ = server.await;
    info!("Server shut down, application exiting");
    Ok(())
}