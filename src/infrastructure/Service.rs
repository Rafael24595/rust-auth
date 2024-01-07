use base64::{Engine as _, engine::general_purpose};

use crate::domain::Service;
use crate::domain::{Services,Key};
use crate::infrastructure::{DtoKey, DtoService};
use crate::commons::configuration::Configuration;

pub(crate) async fn nodekey() -> Result<String, String> {
    let crypto = Configuration::instance().crypto;
    return crypto.read_public();
}

pub(crate) async fn subscribe(code: String, host: String, dto: DtoService::DtoService) -> Result<(), String> {
    let o_service = Services::find(&code.as_str());
    if o_service.is_some() {
        return Err(String::from("Service already registered."));
    }

    let validation = valide_message(dto.clone());
    if validation.is_err() {
        return Err(validation.err().unwrap());
    }

    let crypto = Configuration::instance().crypto;
    let encrypted_message = general_purpose::STANDARD.decode(dto.pass_key).unwrap();
    let r_uuid = crypto.decrypt_message(&encrypted_message);
    if r_uuid.is_err() {
        return Err(r_uuid.err().unwrap());
    }

    let uuid = r_uuid.unwrap();

    let is_authorized = Configuration::find_active_token(uuid);
    if let Err(status) = is_authorized {
        return Err(String::from("Token is not authorized. Status: ") + status.to_string());
    }

    let service = Service::new(code, host, dto.end_point_status, dto.end_point_key);
    Services::insert_service(service);

    return Ok(());
}

pub(crate) async fn status(service: String) -> Result<(), (u16, String)> {
    let o_service = Services::find(service.as_str());
    if o_service.is_some() {
        let service_data = o_service.unwrap();
        let url = service_data.uri() + &service_data.end_point_status();
        let response = reqwest::get(url).await;

        if response.is_err() {
            return Err((reqwest::StatusCode::BAD_REQUEST.as_u16(), response.err().unwrap().to_string()));
        }

        let response_data = response.unwrap();
        let status = response_data.status();
        
        if !status.is_success() {
            let text = response_data.text().await.unwrap_or_default();
            return Err((status.as_u16(), text));
        } 

        return Ok(());
    }

    return Err((reqwest::StatusCode::BAD_REQUEST.as_u16(), "Service is not defined".to_string()));
}

pub(crate) async fn key(service: String) -> Result<(String), (u16, String)> {
    let o_service = Services::find(service.as_str());
    if o_service.is_some() {

        let mut service_data = o_service.unwrap();
        if service_data.has_key() && !service_data.key().unwrap().is_expired() {
            return Ok(service_data.key().unwrap().key());
        }
        
        let url = service_data.uri() + &service_data.end_point_key();
        let response = reqwest::get(url).await;

        if response.is_err() {
            return Err((reqwest::StatusCode::BAD_REQUEST.as_u16(), response.err().unwrap().to_string()));
        }

        let response_data = response.unwrap();
        let status = response_data.status();
        
        if !status.is_success() {
            let text = response_data.text().await.unwrap_or_default();
            return Err((status.as_u16(), text));
        }

        let r_dto_key: Result<DtoKey::DtoKey, reqwest::Error> = response_data.json().await;

        if r_dto_key.is_ok() {
            let dto_key = r_dto_key.unwrap();
            let key = Key::new(dto_key.public, dto_key.expires);
            service_data.update_key(key);
            Services::update(service_data);
            return Ok(key.key());
        }

        return Err((reqwest::StatusCode::INTERNAL_SERVER_ERROR.as_u16(), r_dto_key.err().unwrap().to_string()));
    }

    return Err((reqwest::StatusCode::BAD_REQUEST.as_u16(), "Service not defined".to_string()));
}

fn valide_message(dto: DtoService::DtoService) -> Result<(), String> {
    if let Ok(uuid) = Configuration::includes_active_token(dto.pass_key.clone()) {
        if let Some(token) = Configuration::deprecate_token(uuid) {
            println!("New token created: {}", token.uuid());
        }
        return Err(String::from("Key exposed. Key has been deprecated."));
    }
    return Ok(());
}