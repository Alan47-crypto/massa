#![feature(ip)]

pub mod config;
pub mod consensus;
pub mod crypto;
pub mod network;
pub mod protocol;
pub mod structures;
use crate::network::network_controller::NetworkController;
use crate::protocol::protocol_controller::{ProtocolController, ProtocolEvent, ProtocolEventType};
use log::error;
use std::error::Error;
use tokio::fs::read_to_string;

type BoxResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

async fn run(cfg: config::Config) -> BoxResult<()> {
    let network = NetworkController::new(&cfg.network).await?;

    // launch network controller
    let mut protocol = ProtocolController::new(&cfg.protocol, network).await?;

    // loop over messages
    loop {
        tokio::select! {
            ProtocolEvent(source_node_id, evt) = protocol.wait_event() => match evt {
                ProtocolEventType::ReceivedTransaction(data) => log::info!("reveice transcation with data:{}", data),
                ProtocolEventType::ReceivedBlock(block) => log::info!("reveice a block {:?} from node {:?}", block, source_node_id),
                ProtocolEventType::AskedBlock(hash) => log::info!("Node {:?} asked for block {:?}", source_node_id, hash),
             }
        }
    }

    /* TODO uncomment when it becomes reachable again
    if let Err(e) = protocol.stop().await {
        warn!("graceful protocol shutdown failed: {}", e);
    }
    Ok(())
    */
}

#[tokio::main]
async fn main() {
    // load config
    let config_path = "config/config.toml";
    let cfg = config::Config::from_toml(&read_to_string(config_path).await.unwrap()).unwrap();

    // setup logging
    stderrlog::new()
        .module(module_path!())
        .verbosity(cfg.logging.level)
        .timestamp(stderrlog::Timestamp::Millisecond)
        .init()
        .unwrap();

    match run(cfg).await {
        Ok(_) => {}
        Err(e) => {
            error!("error in program root: {}", e);
        }
    }
}
