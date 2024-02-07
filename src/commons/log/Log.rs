use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::commons::log::{LogConsole, DependencyLog::DependencyLog};

use super::{LogEvent, LogFormatter};

lazy_static! {
    static ref INSTANCE: Mutex<Option<Box<dyn DependencyLog>>> = Mutex::new(None);
}

pub(crate) fn initialize(code: String) {
    let mut instance = INSTANCE.lock().expect("Could not lock mutex");
    let mut dependency: Option<Box<dyn DependencyLog>> = None;
    match code.as_str() {
        LogConsole::CODE |_ => dependency = Some(default())
    }
    *instance = dependency;
}

pub(crate) fn log_info(message: String) {
    let instance = INSTANCE.lock().expect("Could not lock mutex");
    let event = LogEvent::new(LogEvent::LogTag::INFO, message);
    instance.as_ref().unwrap_or(&default()).log(event);
}

pub(crate) fn log_warn(message: String) {
    let instance = INSTANCE.lock().expect("Could not lock mutex");
    let event = LogEvent::new(LogEvent::LogTag::WARN, message);
    instance.as_ref().unwrap_or(&default()).log(event);
}

pub(crate) fn log_error(message: String) {
    let instance = INSTANCE.lock().expect("Could not lock mutex");
    let event = LogEvent::new(LogEvent::LogTag::ERROR, message);
    instance.as_ref().unwrap_or(&default()).log(event);
}

fn default() -> Box<dyn DependencyLog> {
    let formater = Box::new(LogFormatter::new());
    return Box::new(LogConsole::new(formater));
}