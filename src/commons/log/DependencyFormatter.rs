use super::LogEvent::LogEvent;

pub(crate) trait DependencyFormatter: Send {
    fn format(&self, event: LogEvent) -> String;
}