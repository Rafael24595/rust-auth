use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::StatusCode;

use crate::commons::crypto::modules::asymmetric::AsymmetricManager;
use crate::commons::exception::AuthenticationApiException;
use crate::infrastructure::dto::{DtoPubKeyRequest, DtoPubKeyResponse};

use super::AsymmetricManager::AsymmetricManager as _;
use super::Utils;

#[derive(Clone)]
pub struct AsymmetricPublic {
    pubkey: String,
    module: String,
    format: String,
    pass_phrase: String,
    expires: u128,
}

pub(crate) fn new(pubkey: String, module: String, format: String, pass_phrase: String, expires_range: u128, expires: u128) -> AsymmetricPublic {
    AsymmetricPublic {
        pubkey,
        module,
        format,
        pass_phrase,
        expires,
    }
}

pub(crate) fn from_dto(dto: DtoPubKeyRequest::DtoPubKeyRequest) -> AsymmetricPublic {
    return AsymmetricPublic {
        pubkey: dto.key,
        module: dto.module,
        format: dto.format,
        pass_phrase: dto.pass_phrase,
        expires: dto.expires,
    }
}

impl AsymmetricPublic {

    pub fn module(&self) -> String {
        return self.module.clone();
    }
    
    pub fn format(&self) -> String {
        return self.format.clone();
    }

    pub fn pass_phrase(&self) -> String {
        return self.pass_phrase.clone();
    }

    fn find_manager(&self) -> Result<impl AsymmetricManager::AsymmetricManager, String> {
        return Utils::find_manager(self.module.clone(), self.format.clone(), self.pass_phrase.clone());
    }

    pub fn encrypt_message(&self, message: &[u8]) -> Result<Vec<u8>, AuthenticationApiException::AuthenticationApiException> {
        let module = self.find_manager();
        if module.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), module.err().unwrap().to_string()));
        }

        return module.unwrap().encrypt(self.pubkey.clone(), message);
    }
    
    pub fn as_dto(&self) -> DtoPubKeyResponse::DtoPubKeyResponse {
        return DtoPubKeyResponse::new(
            self.pubkey.clone(),
            self.module.clone(), 
            self.format.clone(), 
            self.pass_phrase.clone(), 
        );
    }

    pub fn is_expired(&self) -> bool {
        let current_time = SystemTime::now();
        let duration_since_epoch = current_time.duration_since(UNIX_EPOCH).expect("Critical error.");
        let milliseconds = duration_since_epoch.as_millis();

        return self.expires < milliseconds;
    }

}