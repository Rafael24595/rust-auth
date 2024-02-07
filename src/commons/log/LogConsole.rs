use super::{DependencyLog::DependencyLog, LogEvent::LogEvent, DependencyFormatter::DependencyFormatter};

pub const CODE: &str = "LOG_CONSOLE";

pub struct LogConsole {
    formater: Box<dyn DependencyFormatter>
}

pub(crate) fn new(formater: Box<dyn DependencyFormatter>) -> impl DependencyLog {
    return LogConsole {
        formater
    };
}

impl DependencyLog for LogConsole {

    fn log(&self, event: LogEvent) {
        let message = self.formater.format(event);
        println!("{}", message)
    }

}