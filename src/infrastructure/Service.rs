use base64::{Engine as _, engine::general_purpose};
use reqwest::StatusCode;

use crate::commons::exception::AuthenticationApiException;
use crate::domain::Service;
use crate::domain::{Services,Key};
use crate::infrastructure::{DtoPubKeyRequest, DtoService, DtoPubKeyResponse};
use crate::commons::configuration::Configuration;

pub(crate) async fn nodekey() -> Result<DtoPubKeyResponse::DtoPubKeyResponse, AuthenticationApiException::AuthenticationApiException> {
    let crypto = Configuration::instance().crypto;

    let key = crypto.read_public();
    if key.is_err() {
        return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), String::from("Could not read public key.")));
    }

    let dto = DtoPubKeyResponse::new(
        key.unwrap(), 
        crypto.module(), 
        crypto.format(), 
        crypto.pass_phrase());
    return Ok(dto);
}

pub(crate) async fn subscribe(code: String, dto: DtoService::DtoService) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
    let o_service = Services::find(&code.as_str());
    if o_service.is_some() {
        let message = String::from("Service already registered.");
        return Err(AuthenticationApiException::new(StatusCode::CONFLICT.as_u16(), message));
    }

    let validation = valide_message(dto.clone());
    if validation.is_err() {
        let message = validation.err().unwrap();
        return Err(AuthenticationApiException::new(StatusCode::BAD_REQUEST.as_u16(), message));
    }

    let crypto = Configuration::instance().crypto;
    let encrypted_message = general_purpose::STANDARD.decode(dto.pass_key).unwrap();
    let r_vec_uuid = crypto.decrypt_message(&encrypted_message);
    if r_vec_uuid.is_err() {
        return Err(r_vec_uuid.err().unwrap());
    }

    let r_uuid = String::from_utf8(r_vec_uuid.unwrap());
    if r_uuid.is_err() {
        return Err(AuthenticationApiException::new(StatusCode::UNAUTHORIZED.as_u16(), r_uuid.err().unwrap().to_string()));
    }

    let is_authorized = Configuration::find_active_token(r_uuid.unwrap());
    if let Err(status) = is_authorized {
        let message = String::from("Token is not authorized. Status: ") + status.to_string();
        return Err(AuthenticationApiException::new(StatusCode::UNAUTHORIZED.as_u16(), message));
    }

    let service = Service::new(code.clone(), dto.host, dto.end_point_status, dto.end_point_key);
    Services::insert_service(service);

    let token = Configuration::instance().crypto.sign(code);
    //TODO: Remove after testing.
    //let result = Configuration::instance().crypto.verify(token.clone().unwrap());

    if token.is_err() {
        return Err(token.err().unwrap());
    }

    return Ok(token.unwrap());
}

pub(crate) async fn status(service: String) -> Result<(), AuthenticationApiException::AuthenticationApiException> {
    let o_service = Services::find(service.as_str());
    if o_service.is_some() {
        let service_data = o_service.unwrap();
        let url = service_data.uri() + &service_data.end_point_status();
        let response = reqwest::get(url).await;

        if response.is_err() {
            let message = response.err().unwrap().to_string();
            return Err(AuthenticationApiException::new(StatusCode::BAD_REQUEST.as_u16(), message));
        }

        let response_data = response.unwrap();
        let status = response_data.status();
        
        if !status.is_success() {
            let text = response_data.text().await.unwrap_or_default();
            return Err(AuthenticationApiException::new(status.as_u16(), text));
        } 

        return Ok(());
    }

    return Err(AuthenticationApiException::new(StatusCode::BAD_REQUEST.as_u16(), String::from("Service is not defined.")));
}

pub(crate) async fn key(service: String) -> Result<DtoPubKeyResponse::DtoPubKeyResponse, AuthenticationApiException::AuthenticationApiException> {
    let o_service = Services::find(service.as_str());
    if o_service.is_some() {
        let mut service_data = o_service.unwrap();
        if service_data.has_key() && !service_data.key().unwrap().is_expired() {
            return Ok(service_data.key().unwrap().as_dto());
        }
        
        let url = service_data.uri() + &service_data.end_point_key();
        let response = reqwest::get(url).await;

        if response.is_err() {
            let message = response.err().unwrap().to_string();
            return Err(AuthenticationApiException::new(StatusCode::BAD_REQUEST.as_u16(), message));
        }

        let response_data = response.unwrap();
        let status = response_data.status();
        
        if !status.is_success() {
            let message = response_data.text().await.unwrap_or_default();
            return Err(AuthenticationApiException::new(status.as_u16(), message));
        }

        let r_dto_key: Result<DtoPubKeyRequest::DtoPubKeyRequest, reqwest::Error> = response_data.json().await;

        if r_dto_key.is_ok() {
            let key = Key::from_dto(r_dto_key.unwrap());
            service_data.update_key(key.clone());
            Services::update(service_data);
            return Ok(key.as_dto());
        }

        let message = r_dto_key.err().unwrap().to_string();
        return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), message));
    }

    return Err(AuthenticationApiException::new(StatusCode::BAD_REQUEST.as_u16(), String::from("Service not defined.")));
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