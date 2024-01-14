use base64::{engine::general_purpose, Engine};
use reqwest::StatusCode;

use crate::commons::exception::AuthenticationApiException;

use super::Payload;

const SEPARATOR: &str = ";";

#[derive(Clone, Debug)]
pub struct ServiceToken {
    sign: Vec<u8>,
    payload: Payload::Payload
}

pub(crate) fn new(sign: Vec<u8>, payload: Payload::Payload) -> ServiceToken {
    ServiceToken {
        sign,
        payload
    }
}

pub(crate) fn from_string(token: String) -> Result<ServiceToken, AuthenticationApiException::AuthenticationApiException> {
    let fragments: Vec<&str> = token.split(";").collect();
    if fragments.len() != 2 {
        return malformed_exception();
    }

    let r_sign = &general_purpose::STANDARD.decode(fragments.get(0).unwrap().as_bytes());
    if r_sign.is_err() {
        return malformed_exception();
    }

    let r_json_payload = String::from_utf8(general_purpose::STANDARD.decode(fragments.get(1).unwrap().as_bytes()).unwrap());
    if r_json_payload.is_err() {
        return malformed_exception();
    }

    let r_payload: Result<Payload::Payload, serde_json::Error> = serde_json::from_str(&r_json_payload.unwrap());
    if r_payload.is_err() {
        return malformed_exception();
    }

    return Ok(ServiceToken {
        sign: r_sign.clone().unwrap(),
        payload: r_payload.unwrap()
    });
}

fn malformed_exception() -> Result<ServiceToken, AuthenticationApiException::AuthenticationApiException> {
    return Err(AuthenticationApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Malformed token.")));
}

impl ServiceToken {
    
    pub fn to_string(&self) -> String {
        let payload_json = serde_json::to_string(&self.payload).expect("Failed to serialize to JSON");
        let payload_64 = general_purpose::STANDARD.encode(payload_json.as_bytes());
        let sign_64 = general_purpose::STANDARD.encode(self.sign.clone());

        return sign_64 + SEPARATOR + &payload_64;
    }

    pub fn sign(&self) -> Vec<u8> {
        return self.sign.clone();
    }

    pub fn payload(&self) -> Payload::Payload {
        return self.payload.clone();
    }

}