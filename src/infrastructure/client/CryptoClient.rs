use reqwest::{header::CONTENT_TYPE, Response, StatusCode};

use crate::{commons::{configuration::Configuration, exception::{AuthenticationApiException, ErrorCodes::ErrorCodes}}, domain::{Service, Services}, infrastructure::{client::{CryptoRequest, CryptoResponse}, dto::DtoPubKeyRequest}};

#[derive(Clone, Debug)]
pub struct CryptoClient {
    request: CryptoRequest::CryptoRequest,
    response: Option<CryptoResponse::CryptoResponse>,
}

pub(crate) fn new(request: CryptoRequest::CryptoRequest, response: Option<CryptoResponse::CryptoResponse>) -> CryptoClient {
    return CryptoClient {
        request,
        response
    };
}

pub(crate) fn from_request(request: CryptoRequest::CryptoRequest) -> CryptoClient {
    return new(request, None);
}

pub(crate) async fn status(service: Service::Service) -> Result<(), AuthenticationApiException::AuthenticationApiException> {
    let mut request = CryptoRequest::new();
    request.set_service(service.code());
    request.set_method(String::from("GET"));
    request.set_path(service.end_point_status());

    let r_response = new(request, None).launch().await;
    if r_response.is_err() {
        return Err(r_response.err().unwrap());
    }
    
    let response = r_response.unwrap();
    if !response.is_success() {
        let message = String::from_utf8_lossy(&response.body()).to_string();
        return Err(AuthenticationApiException::new(
            response.status(),
            ErrorCodes::CLIDT001,
            message));
    }
    
    return Ok(());
}

pub(crate) async fn key(service: Service::Service) -> Result<DtoPubKeyRequest::DtoPubKeyRequest, AuthenticationApiException::AuthenticationApiException> {
    let mut request = CryptoRequest::new();
    request.set_service(service.code());
    request.set_method(String::from("GET"));
    request.set_path(service.end_point_key());

    let response = new(request, None).launch().await;
    if response.is_err() {
        return Err(response.err().unwrap());
    }
    
    let dto = serde_json::from_slice(&response.unwrap().body());
    if dto.is_err() {
        return Err(AuthenticationApiException::new(
            StatusCode::BAD_REQUEST.as_u16(), 
            ErrorCodes::CLIDT002,
            dto.err().unwrap().to_string()));
    }

    return Ok(dto.unwrap());
}

impl CryptoClient {
    
    pub fn request(&self) -> CryptoRequest::CryptoRequest {
        return self.request.clone();
    }

    pub fn response(&self) -> Option<CryptoResponse::CryptoResponse> {
        return self.response.clone();
    }

    pub fn is_launched(&self) -> bool {
        return self.response.is_some();
    }

    pub async fn launch(&mut self) -> Result<CryptoResponse::CryptoResponse, AuthenticationApiException::AuthenticationApiException> {
        let o_service = Services::find(&&self.request.service());
        if o_service.is_none() {
            return Err(AuthenticationApiException::new(
                StatusCode::BAD_REQUEST.as_u16(),
                ErrorCodes::CLIUA001,
                String::from("Service is not defined.")));
        }
        
        let service = o_service.unwrap();
        let petition = self.parse_request(service.clone(), self.request.clone())?;

        let response = petition.send().await;
        if response.is_err() {
            return Err(AuthenticationApiException::new(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                ErrorCodes::CLIDT003,
                response.err().unwrap().to_string()));
        }

        return self.parse_response(service, response.unwrap()).await;
    }

    fn parse_request(&self, service: Service::Service, request: CryptoRequest::CryptoRequest) -> Result<reqwest::RequestBuilder, AuthenticationApiException::AuthenticationApiException> {        
        let client = reqwest::Client::new();
        
        let host = service.uri();
        let uri = host + "/" + &request.uri();
        let method = request.method();

        let mut petition;
        match method.as_str() {
            "GET" => petition = client.get(uri),
            "HEAD" => petition = client.head(uri),
            "POST" => petition = client.post(uri),
            "PUT" => petition = client.put(uri),
            "DELETE" => petition = client.delete(uri),
            "PATCH" => petition = client.patch(uri),
            "OPTION" | "TRACE" => 
                return Err(AuthenticationApiException::new(
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(), 
                    ErrorCodes::SYSIN001,
                    String::from("Method not allowed yet."))),
            _ => 
                return Err(AuthenticationApiException::new(
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    ErrorCodes::SYSIN003,
                    String::from("Method not found"))),
        };

        if method != "GET" {
            let v_body = request.body();
            let symmetric = service.symmetric_key()?;
            let encrypted = symmetric.encrypt_message(&v_body)?;
            petition = petition.body(encrypted);
        }

        petition = petition.header(CONTENT_TYPE, "text/plain");

        for header in request.headers() {
            let key = header.key();
            if !key.eq_ignore_ascii_case(Configuration::COOKIE_NAME) {
                for value in header.values() {
                    petition = petition.header(key.clone(), value);
                }
            }
        }

        return Ok(petition);
    }

    pub async fn parse_response(&mut self, service: Service::Service, response: Response) -> Result<CryptoResponse::CryptoResponse, AuthenticationApiException::AuthenticationApiException> {
        let mut crypto_response = CryptoResponse::new();

        crypto_response.set_status(response.status().as_u16());

        for (o_name, b_value) in response.headers() {
            let name = o_name.to_string();
            let value = b_value.to_str().unwrap().to_string();
            crypto_response.add_header_parameter_tuple(name, value);
        }

        let mut body = Vec::new();
        let b_body = response.text().await;
        if b_body.is_ok() {
            let symmetric = service.symmetric_key()?;
            let decrypted = symmetric.decrypt_message(b_body.unwrap().as_bytes())?;
            body = decrypted.as_bytes().to_vec();
        }

        crypto_response.set_body(body);

        return Ok(crypto_response);
    }

}