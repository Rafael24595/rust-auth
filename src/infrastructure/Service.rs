use base64::{Engine as _, engine::general_purpose};
use reqwest::StatusCode;

use crate::commons::crypto::modules::asymmetric::AsymmetricPublic;
use crate::commons::exception::AuthenticationApiException;
use crate::domain::Service;
use crate::domain::Services;
use crate::infrastructure::{DtoService, DtoPubKeyResponse};
use crate::commons::configuration::Configuration;

use crate::infrastructure::CryptoClient;
use crate::infrastructure::entity::{CryptoRequest, CryptoResponse};

pub(crate) async fn nodekey() -> Result<DtoPubKeyResponse::DtoPubKeyResponse, AuthenticationApiException::AuthenticationApiException> {
    let crypto = Configuration::instance().crypto;
    return Ok(crypto.read_public());
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
    let r_vec_uuid = crypto.asymmetric_key_pair().decrypt_message(&encrypted_message);
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

    let token = Configuration::instance().crypto.asymmetric_key_pair().sign(code);

    if token.is_err() {
        return Err(token.err().unwrap());
    }

    return Ok(token.unwrap());
}

pub(crate) async fn status(service: String) -> Result<(), AuthenticationApiException::AuthenticationApiException> {
    let o_service = Services::find(service.as_str());
    if o_service.is_some() {
        let service_data = o_service.unwrap();
        let response = CryptoClient::status(service_data).await;

        if response.is_err() {
            return Err(response.err().unwrap());
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
        
        let response = CryptoClient::key(service_data.clone()).await;
        if response.is_err() {
            return Err(response.err().unwrap());
        }
    
        let key = AsymmetricPublic::from_dto(response.unwrap());
        service_data.update_key(key.clone());
        Services::update(service_data);
        
        return Ok(key.as_dto());
    }

    return Err(AuthenticationApiException::new(StatusCode::BAD_REQUEST.as_u16(), String::from("Service not defined.")));
}

pub(crate) async fn resolve(crypto_request: CryptoRequest::CryptoRequest) -> Result<CryptoResponse::CryptoResponse, AuthenticationApiException::AuthenticationApiException> {
    let mut client = CryptoClient::from_request(crypto_request);
    return client.launch().await;
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