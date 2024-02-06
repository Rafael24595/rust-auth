use axum::{body::{to_bytes, Body}, extract::Request, http::{Response, StatusCode}, response::IntoResponse};

use crate::{commons::{crypto::ServiceToken, exception::{AuthenticationApiException, ErrorCodes::ErrorCodes}}, domain::{Service, Services}, infrastructure::client::{CryptoRequest, CryptoResponse}};

pub(crate) async fn parse_request(web_service: Service::Service, service: String, path: String, request: Request) -> Result<CryptoRequest::CryptoRequest, AuthenticationApiException::AuthenticationApiException>  {
    let method = request.method().to_string();
    let headers = request.headers().clone();
    let uri = request.uri().clone();
    let query = uri.query().unwrap_or_default();
    let mut body = Vec::new();
    let b_body = to_bytes(request.into_body(), usize::MAX).await;
    if b_body.is_ok() {
        let symmetric_key = web_service.symmetric_key()?;
        let decrypted = symmetric_key.decrypt_message(&b_body.unwrap().to_vec());
        if decrypted.is_err() {
            return Err(decrypted.err().unwrap()); 
        }
        body = decrypted.unwrap().as_bytes().to_vec();
    }

    let mut crypto_request = CryptoRequest::new();
    crypto_request.set_method(method);
    crypto_request.set_service(service);
    crypto_request.set_path(path);
    crypto_request.set_query(query);
    crypto_request.set_body(body);

    for (o_name, b_value) in headers {
        let name = o_name.unwrap().to_string();
        let value = b_value.to_str().unwrap().to_string();
        crypto_request.add_header_parameter_tuple(name, value);
    }

    return Ok(crypto_request);
}

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
    let symmetric_key = web_service.symmetric_key()?;
    let encrypted = symmetric_key.encrypt_message(&b_body);
    if encrypted.is_err() {
        return Err(encrypted.err().unwrap()); 
    }
    let body = encrypted.unwrap().as_bytes().to_vec();

    let r_response = response.body(Body::from(body));
    if r_response.is_err() {
        let error = r_response.err().unwrap();
        return Err(AuthenticationApiException::new(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(), 
            ErrorCodes::SYSIN002,
            error.to_string()));    
    }

    return Ok(r_response.unwrap());
}

pub(crate) fn token_service(o_token: Option<&axum::http::HeaderValue>) -> Result<Service::Service, AuthenticationApiException::AuthenticationApiException>  {
    if o_token.is_none() {
        return Err(
            AuthenticationApiException::new(StatusCode::UNAUTHORIZED.as_u16(), 
            ErrorCodes::CLIUA002,
            String::from("Token not found")));
    }
    let token = o_token.unwrap().to_str().unwrap().to_string();

    let service_token = ServiceToken::from_string(token)?;
    let o_service = Services::find(&service_token.payload().service);
    if o_service.is_none() {
        return Err(AuthenticationApiException::new(
            StatusCode::UNAUTHORIZED.as_u16(),
            ErrorCodes::CLIUA001,
            String::from("Service not found")));
    }
    
    return Ok(o_service.unwrap());
}

impl IntoResponse for AuthenticationApiException::AuthenticationApiException {

    fn into_response(self) -> Response<Body> {
        Response::builder()
        .status(self.status())
        .header(AuthenticationApiException::EXCEPTION_HEADER, self.error_code().code())
        .body(Body::from(self.message()))
        .unwrap()
    }

}