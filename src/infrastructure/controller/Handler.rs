use std::net::SocketAddr;

use axum::{extract::{ConnectInfo, Request}, http::{HeaderMap, HeaderValue, StatusCode}, middleware::Next, response::{IntoResponse, Response}};

use crate::commons::{configuration::Configuration, exception::{AuthenticationApiException, ErrorCodes::ErrorCodes}};

pub(crate) async fn auth_handler(headers: HeaderMap, request: Request, next: Next) -> Result<Response, impl IntoResponse> {
    let o_token = headers.get(String::from(Configuration::COOKIE_NAME));
    if o_token.is_none() {
        let error = AuthenticationApiException::new(
            StatusCode::UNAUTHORIZED.as_u16(), 
            ErrorCodes::CLIUA002,
            String::from("Token not found"));
        return Err(error.into_response());
    }
    let token = o_token.unwrap().to_str().unwrap().to_string();
    let config = Configuration::instance().crypto.asymmetric_key_pair();
    let r_validation = config.verify(token);
    if r_validation.is_err() {
        return Err(r_validation.err().unwrap().into_response());
    }
    let validation = r_validation.unwrap();

    let mut response = next.run(request).await;

    if validation.is_some() {
        let refresh = validation.unwrap();
        let headers = response.headers_mut();
        let header = HeaderValue::from_str(&(Configuration::COOKIE_NAME.to_owned() + &"=" + &refresh.to_string()));
        if header.is_err() {
            //TODO: Log.
        }
        headers.insert("Set-Cookie", header.unwrap());
    }

    return Ok(response);
}

pub(crate) async fn client_tracer_handler(ConnectInfo(addr): ConnectInfo<SocketAddr>, headers: HeaderMap, request: Request, next: Next) -> Result<Response, StatusCode> {
    let ip = addr.ip().to_string();
    let port = addr.port();
    let mut ipv = String::from("ipv4");
    if addr.is_ipv6() {
        ipv = String::from("ipv6");
    }
    let pass_token = headers.get(String::from(Configuration::COOKIE_NAME));

    let response = next.run(request).await;

    return Ok(response);
}