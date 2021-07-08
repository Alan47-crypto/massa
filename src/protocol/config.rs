use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ProtocolConfig {
    pub message_timeout: std::time::Duration,
    pub ask_peer_list_interval: std::time::Duration,
}
