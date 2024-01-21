use reqwest::StatusCode;

use crate::{infrastructure::entity::{CryptoRequest, CryptoResponse}, domain::Services, commons::{exception::AuthenticationApiException, configuration::Configuration}};

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
            return Err(AuthenticationApiException::new(StatusCode::BAD_REQUEST.as_u16(), String::from("Service is not defined.")));
        }
        
        let client = reqwest::Client::new();
        
        let host = o_service.unwrap().uri();
        let uri = host + "/" + &self.request.uri();
        let method = self.request.method();

        let mut petition;
        match method.as_str() {
            "GET" => petition = client.get(uri),
            "HEAD" => petition = client.head(uri),
            "POST" => petition = client.post(uri),
            "PUT" => petition = client.put(uri),
            "DELETE" => petition = client.delete(uri),
            "PATCH" => petition = client.patch(uri),
            "OPTION" | "TRACE" => 
                return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), String::from("Method not allowed yet."))),
            _ => 
                return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), String::from("Method not found"))),
        };

        if method != "GET" {
            let v_body = self.request.body();
            let body = String::from_utf8_lossy(&v_body);
            petition = petition.body(body.to_string());
        }

        for header in self.request.headers() {
            let key = header.key();
            if !key.eq_ignore_ascii_case(Configuration::COOKIE_NAME) {
                for value in header.values() {
                    petition = petition.header(key.clone(), value);
                }
            }
        }

        let o_response = petition.send().await;
        if o_response.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), o_response.err().unwrap().to_string()));
        }

        let response = o_response.unwrap();        

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
            body = b_body.unwrap().as_bytes().to_vec();
        }

        crypto_response.set_body(body);

        return Ok(crypto_response);
    }

}