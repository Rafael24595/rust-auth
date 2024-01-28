use std::collections::HashMap;

use crate::domain::Service;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref INSTANCE: Mutex<Services> = Mutex::new(new());
}

struct Services {
    services: HashMap<String, Service::Service>
}

fn new() -> Services {
    Services {
        services: HashMap::new()
    }
}

pub(crate) fn find(code: &str) -> Option<Service::Service> {
    let services = INSTANCE.lock().unwrap();
    let service = services.services.get(code);
    if service.is_some() {
        return Some(service.unwrap().clone());
    }
    return None;
}

pub(crate) fn insert(code: String, uri: String, subscription_uuid: String, end_point_status: String, end_point_key: String) -> Service::Service {
    let new_service = Service::new(code.clone(), uri, subscription_uuid, end_point_status, end_point_key);
    INSTANCE.lock().unwrap().services.insert(code.clone(), new_service.clone());
    return new_service;
}

pub(crate) fn insert_service(service: Service::Service) -> Service::Service {
    INSTANCE.lock().unwrap().services.insert(service.code().clone(), service.clone());
    return service;
}

pub(crate) fn update(service: Service::Service) -> Service::Service {
    INSTANCE.lock().unwrap().services.insert(service.code(), service.clone());
    return service;
}