use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub service: String,
    pub expires: u128,
    pub timestamp: u128
}

pub(crate) fn new(service: String, expires_range: u128) -> Payload {
    let current_system_time = SystemTime::now();
    let duration_since_epoch = current_system_time.duration_since(UNIX_EPOCH);
    let timestamp = duration_since_epoch.unwrap_or_default().as_millis();
    Payload {
        service: service,
        expires: timestamp + expires_range,
        timestamp: timestamp
    }
}

impl Payload {
    
    pub fn to_json(&self) -> String {
       return serde_json::to_string(&self).expect("Failed to serialize to JSON");
    }

}