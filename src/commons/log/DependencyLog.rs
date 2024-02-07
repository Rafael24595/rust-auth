use super::LogEvent::LogEvent;

pub(crate) trait DependencyLog: Send {
    fn log(&self, event: LogEvent);
}