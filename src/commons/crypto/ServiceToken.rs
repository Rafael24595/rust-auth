use std::time::{SystemTime, UNIX_EPOCH};

use base64::{engine::general_purpose, Engine};
use reqwest::StatusCode;

use crate::commons::exception::{AuthenticationApiException, ErrorCodes::ErrorCodes};

use super::Payload;

const EXPIRATION_MARGIN: u128 = 240000;

const SEPARATOR: &str = ";";

#[derive(Clone, Debug)]
pub struct ServiceToken {
    hash: Option<Vec<u8>>,
    sign: Vec<u8>,
    payload: Payload::Payload
}

pub(crate) fn new(sign: Vec<u8>, payload: Payload::Payload) -> ServiceToken {
    ServiceToken {
        hash: None,
        sign: sign,
        payload: payload
    }
}

pub(crate) fn from_string(token: String) -> Result<ServiceToken, AuthenticationApiException::AuthenticationApiException> {
    let mut fragments: Vec<&str> = token.split(";").collect();
    fragments.reverse();
    if fragments.len() < 2 {
        return malformed_exception();
    }

    let mut hash = None;
    if fragments.len() == 3 {
        let r_hash = &general_purpose::STANDARD.decode(fragments.pop().unwrap().as_bytes());
        if r_hash.is_err() {
            return malformed_exception();
        }
        hash = Some(r_hash.clone().unwrap());
    }

    let r_sign = &general_purpose::STANDARD.decode(fragments.pop().unwrap().as_bytes());
    if r_sign.is_err() {
        return malformed_exception();
    }

    let r_json_payload = String::from_utf8(general_purpose::STANDARD.decode(fragments.pop().unwrap().as_bytes()).unwrap());
    if r_json_payload.is_err() {
        return malformed_exception();
    }

    let r_payload: Result<Payload::Payload, serde_json::Error> = serde_json::from_str(&r_json_payload.unwrap());
    if r_payload.is_err() {
        return malformed_exception();
    }

    return Ok(ServiceToken {
        hash: hash,
        sign: r_sign.clone().unwrap(),
        payload: r_payload.unwrap()
    });
}

fn malformed_exception() -> Result<ServiceToken, AuthenticationApiException::AuthenticationApiException> {
    return Err(AuthenticationApiException::new(
        StatusCode::UNAUTHORIZED.as_u16(), 
        ErrorCodes::CLIUA003,
        String::from("Malformed token.")));
}

impl ServiceToken {
    
    pub fn to_string(&self) -> String {
        let payload_json = self.payload.to_json();
        let payload_64 = general_purpose::STANDARD.encode(payload_json.as_bytes());
        let sign_64 = general_purpose::STANDARD.encode(self.sign.clone());

        let mut token = String::new();
        if self.hash.is_some() {
            let hash_64 = general_purpose::STANDARD.encode(self.hash.clone().unwrap());
            token = token + &hash_64 + SEPARATOR;
        }

        token = token + &sign_64 + SEPARATOR + &payload_64;

        return token;
    }

    pub fn hash(&self) -> Option<Vec<u8>> {
        return self.hash.clone();
    }

    pub fn sign(&self) -> Vec<u8> {
        return self.sign.clone();
    }

    pub fn payload(&self) -> Payload::Payload {
        return self.payload.clone();
    }

    pub fn set_hash(&mut self, hash: Vec<u8>) {
        self.hash = Some(hash);
    }

    pub fn is_alive(&self) -> Result<(), (bool, AuthenticationApiException::AuthenticationApiException)> {
        let current_system_time = SystemTime::now();
        let duration_since_epoch = current_system_time.duration_since(UNIX_EPOCH);
        let timestamp = duration_since_epoch.unwrap_or_default().as_millis();

        if timestamp > self.payload.expires {
            let refresh = (timestamp - self.payload.expires) < EXPIRATION_MARGIN;
            return Err((refresh, AuthenticationApiException::new(
                StatusCode::UNAUTHORIZED.as_u16(),
                ErrorCodes::CLIFB003,
                String::from("Token has expired."))));
        }

        return Ok(());
    }

}