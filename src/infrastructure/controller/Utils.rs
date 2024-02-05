use axum::{body::Body, http::{Response, StatusCode}, response::IntoResponse};

use crate::{commons::{crypto::ServiceToken, exception::AuthenticationApiException}, domain::{Service, Services}, infrastructure::client::CryptoResponse};

pub(crate) fn parse_response(web_service: Service::Service, crypto_response: CryptoResponse::CryptoResponse) -> Result<Response<Body>, AuthenticationApiException::AuthenticationApiException>  {
    let mut response = Response::builder();
    response = response.status(crypto_response.status());

    for header in crypto_response.headers() {
        let key = header.key();
        for value in header.values() {
            response = response.header(key.clone(), value);
        }
    }
    
    let b_body = crypto_response.body();
    if web_service.symetric_key().is_none() {
        return Err(AuthenticationApiException::new(StatusCode::FORBIDDEN.as_u16(), String::from("Symmetric key not found"))); 
    }
    let decrypted = web_service.symetric_key().unwrap().encrypt_message(&b_body);
    if decrypted.is_err() {
        return Err(AuthenticationApiException::new(StatusCode::FORBIDDEN.as_u16(), decrypted.err().unwrap().message())); 
    }
    let body = decrypted.unwrap().as_bytes().to_vec();

    let r_response = response.body(Body::from(body));
    if r_response.is_err() {
        let error = r_response.err().unwrap();
        return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error.to_string()));    
    }

    return Ok(r_response.unwrap());
}

pub(crate) fn token_service(o_token: Option<&axum::http::HeaderValue>) -> Result<Service::Service, AuthenticationApiException::AuthenticationApiException>  {
    if o_token.is_none() {
        return Err(AuthenticationApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Token not found")));
    }
    let token = o_token.unwrap().to_str().unwrap().to_string();
    let service_token = ServiceToken::from_string(token);
    if service_token.is_err() {
        return Err(AuthenticationApiException::new(StatusCode::UNAUTHORIZED.as_u16(), service_token.err().unwrap().message()));
    }

    let o_service = Services::find(&service_token.unwrap().payload().service);
    if o_service.is_none() {
        return Err(AuthenticationApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Service not found")));
    }
    
    return Ok(o_service.unwrap());
}

impl IntoResponse for AuthenticationApiException::AuthenticationApiException {

    fn into_response(self) -> Response<Body> {
        Response::builder()
        .status(self.status())
        .header(AuthenticationApiException::ExceptionHeader, self.error_code().code())
        .body(Body::from(self.message()))
        .unwrap()
    }

}