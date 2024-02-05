use base64::{engine::general_purpose, Engine};
use reqwest::StatusCode;

use crate::commons::exception::{AuthenticationApiException, ErrorCodes::ErrorCodes};

pub struct AesGcmMessage {
    nonce: Vec<u8>,
    payload: Vec<u8>
}

pub(crate) fn new(nonce: Vec<u8>, payload: Vec<u8>) -> AesGcmMessage {
    return AesGcmMessage {
        nonce, 
        payload
    };
}

pub(crate) fn from_slice(slice: &[u8]) -> Result<AesGcmMessage, AuthenticationApiException::AuthenticationApiException> {
    let fragments: Vec<&[u8]> = slice.split(|&b| b == 59).collect();

    let mut nonce = Vec::new();
    if fragments.len() > 1 {
        let decoded = general_purpose::STANDARD.decode(fragments.first().unwrap().to_vec());
        if decoded.is_err() {
            return Err(AuthenticationApiException::new(
                StatusCode::UNPROCESSABLE_ENTITY.as_u16(), 
                ErrorCodes::CLIFB006,
                decoded.err().unwrap().to_string()));
        }
        nonce = decoded.unwrap();
    }

    let mut payload = Vec::new();
    if fragments.len() > 0 {
        let decoded = general_purpose::STANDARD.decode(fragments.last().unwrap().to_vec());
        if decoded.is_err() {
            return Err(AuthenticationApiException::new(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                ErrorCodes::CLIFB006,
                decoded.err().unwrap().to_string()));
        }
        payload = decoded.unwrap();
    }
    
    return Ok(AesGcmMessage{
        nonce,
        payload
    });
}

impl AesGcmMessage {
    
    pub fn nonce(&self) -> Vec<u8> {
        return self.nonce.clone();
    }

    pub fn payload(&self) -> Vec<u8> {
        return self.payload.clone();
    }

    pub fn to_string(&self) -> String {
        let nonce = String::from_utf8_lossy(&self.nonce).to_string();
        let payload = String::from_utf8_lossy(&self.payload).to_string();
        return nonce + ";" + &payload;
    }

}