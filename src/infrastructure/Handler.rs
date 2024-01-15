use axum::{http::{HeaderMap, StatusCode, HeaderValue}, extract::Request, middleware::Next, response::Response};

use crate::commons::configuration::Configuration;

const COOKIE_NAME: &str = "Pass-Token";

pub(crate) async fn auth_handler(headers: HeaderMap, request: Request, next: Next) -> Result<Response, StatusCode> {
    let o_token = headers.get(String::from(COOKIE_NAME));
    if o_token.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let token = o_token.unwrap().to_str().unwrap().to_string();
    let config = Configuration::instance().crypto;
    let r_validation = config.verify(token);
    if r_validation.is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let validation = r_validation.unwrap();

    let mut response = next.run(request).await;

    if validation.is_some() {
        let refresh = validation.unwrap();
        let headers = response.headers_mut();
        let header = HeaderValue::from_str(&(COOKIE_NAME.to_owned() + &"=" + &refresh.to_string()));
        if header.is_err() {
            //TODO: Log.
        }
        headers.insert("Set-Cookie", header.unwrap());
    }

    return Ok(response);
}