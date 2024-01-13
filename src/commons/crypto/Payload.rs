use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub service: String,
    pub timestamp: u128
}

pub(crate) fn new(service: String) -> Payload {
    let current_system_time = SystemTime::now();
    let duration_since_epoch = current_system_time.duration_since(UNIX_EPOCH);
    let timestamp = duration_since_epoch.unwrap_or_default().as_millis();
    Payload {
        service: service,
        timestamp: timestamp
    }
}