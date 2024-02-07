use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

#[derive(Serialize)]
pub struct LogEvent {
    tag: LogTag,
    message: String,
    timestamp: u128
}

pub(crate) fn new(tag: LogTag, message: String) -> LogEvent {
    let current_system_time = SystemTime::now();
    let duration_since_epoch = current_system_time.duration_since(UNIX_EPOCH);
    let timestamp = duration_since_epoch.unwrap_or_default().as_millis();

    return LogEvent {
        tag,
        message,
        timestamp
    };
}

impl LogEvent {
    
    pub fn tag(&self) -> String {
        return self.tag.as_string();
    }

    pub fn message(&self) -> String {
        return self.message.clone();
    }

    pub fn timestamp(&self) -> u128 {
        return self.timestamp;
    }

}

#[derive(Serialize)]
pub enum LogTag {
    INFO,
    WARN,
    ERROR
}

impl LogTag {
    
    pub fn as_string(&self) -> String {
        match *self {
            LogTag::INFO => String::from("INFO"),
            LogTag::WARN => String::from("WARN"),
            LogTag::ERROR => String::from("ERROR"),
        }
    }

}