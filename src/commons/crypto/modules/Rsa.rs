use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePrivateKey};
use reqwest::StatusCode;

use crate::commons::{crypto::modules::CryptoManager, exception::AuthenticationApiException};

pub const MODULE_CODE: &str = "RSA";

const PEM: &str = "PEM";
const PKCS1: &str = "PKCS1";
const PKCS8: &str = "PKCS1";

#[derive(Clone)]
pub struct Rsa {
    format: String,
    is_pem: bool,
    pass_phrase: String
}

pub(crate) fn new(format: String, pass_phrase: String) -> impl CryptoManager::CryptoManager {
    let format_fragments: Vec<&str> = format.split("&").collect();
    let format_value = format_fragments.first().unwrap().to_string();
    let is_pem = format_fragments.last().unwrap().to_string().eq_ignore_ascii_case(PEM);

    Rsa {
        format: format_value,
        is_pem: is_pem,
        pass_phrase: pass_phrase
    }
}

impl CryptoManager::CryptoManager for Rsa {
    
    fn encrypt(&self, priv_string: String, encrypted_message: &[u8]) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        Ok(String::new())
    }

    fn decrypt(&self, priv_string: String, encrypted_message: &[u8]) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        let priv_key = self.private_key(priv_string);
        if priv_key.is_err() {
            return Err(priv_key.err().unwrap());
        }
    
        let dec_data = priv_key.unwrap().decrypt(Pkcs1v15Encrypt, &encrypted_message).expect("failed to decrypt");
        let result = String::from_utf8(dec_data);
        if result.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::BAD_REQUEST.as_u16(), result.err().unwrap().to_string()));
        }

        Ok(result.unwrap())
    }

}

impl Rsa {
    
    fn private_key(&self, priv_string: String) -> Result<RsaPrivateKey, AuthenticationApiException::AuthenticationApiException> {
        if self.is_pem {
            if self.format.eq_ignore_ascii_case(PKCS1) {
                let priv_key = RsaPrivateKey::from_pkcs1_pem(&priv_string);
                if priv_key.is_err() {
                    return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), priv_key.err().unwrap().to_string()));
                }
                return Ok(priv_key.unwrap());
            }
            if self.format.eq_ignore_ascii_case(PKCS8) {
                let priv_key = RsaPrivateKey::from_pkcs8_pem(&priv_string);
                if priv_key.is_err() {
                    return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), priv_key.err().unwrap().to_string()));
                }
                return Ok(priv_key.unwrap());
            }
        }

        return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), String::from("Invalid format")));
    }

}