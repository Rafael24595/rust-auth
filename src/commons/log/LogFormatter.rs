use chrono::{TimeZone, Utc};

use super::{LogEvent::LogEvent, DependencyFormatter::DependencyFormatter};

pub struct LogFormatter {
}

pub(crate) fn new() -> impl DependencyFormatter {
    return LogFormatter {
    };
}

impl DependencyFormatter for LogFormatter {
    
    fn format(&self, event: LogEvent) -> String {
        let date = self.date_format(event.timestamp());
        return format!("[{}] - {} => {}", event.tag(), date, event.message());
    }

}

impl LogFormatter {
    
    pub fn format(&self, event: LogEvent) -> String {
        let date = self.date_format(event.timestamp());
        return format!("[{}] - {} => {}", event.tag(), date, event.message());
    }

    fn date_format(&self, timestamp: u128) -> String {
        let datetime_utc = Utc.timestamp_millis_opt(timestamp as i64).unwrap();
        return datetime_utc.format("%Y-%m-%d %H:%M:%S").to_string()
    }

}