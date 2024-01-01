use std::collections::HashMap;

use crate::domain::Service;

struct Auth {
    services: HashMap<String, Service::Service>
}

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref INSTANCE: Mutex<Auth> = Mutex::new(new());
}

fn new() -> Auth {
    Auth {
        services: HashMap::new()
    }
}

pub(crate) fn find_service(code: &str) -> Option<Service::Service> {
    let services = INSTANCE.lock().unwrap();
    let service = services.services.get(code);
    if service.is_some() {
        return Some(*service.unwrap());
    }
    return None;
}

pub(crate) fn insert_service(code: String, uri: String) -> Service::Service {
    let new_service = Service::new(code.clone(), uri);
    INSTANCE.lock().unwrap().services.insert(code.clone(), new_service.clone());
    return new_service;
}