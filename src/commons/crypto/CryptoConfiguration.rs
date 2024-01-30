use base64::engine::general_purpose;
use base64::Engine;
use reqwest::StatusCode;

use crate::commons::exception::AuthenticationApiException;
use crate::infrastructure::{DtoPubKeyResponse, DtoService, DtoSuscribePayload};

use super::modules::asymmetric::AsymmetricKeys;
use super::modules::symmetric::{SymmetricKey, SymmetricKeys};

#[derive(Clone)]
pub struct CryptoConfiguration {
    asymmetric_key_pair: AsymmetricKeys::AsymmetricKeys,
    symetric_keys: SymmetricKeys::SymmetricKeys
}

pub(crate) fn new(asymmetric_key_pair: AsymmetricKeys::AsymmetricKeys, symetric_keys: SymmetricKeys::SymmetricKeys) -> CryptoConfiguration {
    CryptoConfiguration {
        asymmetric_key_pair: asymmetric_key_pair,
        symetric_keys: symetric_keys
    }
}

impl CryptoConfiguration {

    pub fn asymmetric_key_pair(&self) -> AsymmetricKeys::AsymmetricKeys {
        return self.asymmetric_key_pair.clone();
    }

    pub fn symmetric_key(&mut self) -> Result<SymmetricKey::SymmetricKey, AuthenticationApiException::AuthenticationApiException> {
        return self.symetric_keys.find();
    }

    pub fn read_public(&self) -> DtoPubKeyResponse::DtoPubKeyResponse {
        return self.asymmetric_key_pair.public_key();
    }

    pub fn decrypt_suscribe_payload(&self, payload: DtoSuscribePayload::DtoSuscribePayload) -> Result<DtoService::DtoService, AuthenticationApiException::AuthenticationApiException> {
        let encrypted_payload = general_purpose::STANDARD.decode(payload.payload).unwrap();
        let decrypted_payload = self.asymmetric_key_pair.decrypt_message(&encrypted_payload)?;

        let payload_string =  String::from_utf8(decrypted_payload);
        if payload_string.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::UNAUTHORIZED.as_u16(), payload_string.err().unwrap().to_string()));
        }

        let r_payload: Result<DtoService::DtoService, serde_json::Error> = serde_json::from_str(&payload_string.unwrap());
        if r_payload.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::UNAUTHORIZED.as_u16(), r_payload.err().unwrap().to_string()));
        }

        return Ok(r_payload.unwrap());
    }

}