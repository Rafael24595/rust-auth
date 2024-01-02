use crate::domain::{Auth,Key};
use crate::infrastructure::DtoKey;

pub(crate) async fn status(service: String) -> Result<(), (u16, String)> {
    let o_service = Auth::find_service(service.as_str());
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
    let o_service = Auth::find_service(service.as_str());
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
            Auth::update_service(service_data);
            return Ok(key.key());
        }

        return Err((reqwest::StatusCode::INTERNAL_SERVER_ERROR.as_u16(), r_dto_key.err().unwrap().to_string()));
    }

    return Err((reqwest::StatusCode::BAD_REQUEST.as_u16(), "Service not defined".to_string()));
}