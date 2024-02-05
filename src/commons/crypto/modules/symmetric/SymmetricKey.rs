use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::StatusCode;

use crate::commons::crypto::modules::symmetric::Utils;
use crate::commons::exception::AuthenticationApiException;

use crate::commons::crypto::modules::symmetric::AesBytes;
use crate::commons::exception::AuthenticationAppException;
use crate::infrastructure::dto::DtoSymetricKey;

use super::{Aes, AesGcm, SymmetricManager};
use super::SymmetricManager::SymmetricManager as _;

#[derive(Clone)]
pub struct SymmetricKey {
    module: String,
    key: Vec<u8>,
    format: String,
    expires: u128,
    timestamp: u128,
    status: SymmetricKeyState
}

pub(crate) fn new(module: String, format: String, expires: u128) -> Result<SymmetricKey, AuthenticationAppException::AuthenticationAppException> {
    let key = generate_key(module.clone(), format.clone());
    if key.is_err() {
        return Err(key.err().unwrap());
    }

    return _new(key.unwrap(), module, format, expires);
}

fn _new(key: Vec<u8>, module: String, format: String, expires: u128) -> Result<SymmetricKey, AuthenticationAppException::AuthenticationAppException> {
    let current_system_time = SystemTime::now();
    let duration_since_epoch = current_system_time.duration_since(UNIX_EPOCH);
    let timestamp = duration_since_epoch.unwrap_or_default().as_millis();

    let data = SymmetricKey {
        module: module,
        key: key,
        format: format,
        expires: expires,
        timestamp: timestamp,
        status: SymmetricKeyState::ACTIVE
    };

    return Ok(data);
}

pub(crate) fn from(from: SymmetricKey) -> Result<SymmetricKey, AuthenticationAppException::AuthenticationAppException> {
    return new(from.module, from.format, from.expires);
}

pub(crate) fn from_dto(dto: DtoSymetricKey::DtoSymetricKey) -> Result<SymmetricKey, AuthenticationApiException::AuthenticationApiException> {
    let result = _new(dto.key.as_bytes().to_vec(), dto.module, dto.format, dto.expires);
    if result.is_err() {
        return Err(AuthenticationApiException::new(StatusCode::UNAUTHORIZED.as_u16(), result.err().unwrap().to_string()));
    }
    return Ok(result.unwrap());
}

fn generate_key(module: String, format: String) -> Result<Vec<u8>, AuthenticationAppException::AuthenticationAppException> {
    match module.as_str() {
        AesGcm::MODULE_CODE => {
            let size = format.parse::<usize>();
            if size.is_err() {
                return Err(AuthenticationAppException::new(size.err().unwrap().to_string()));
            }

            let bytes = AesBytes::from_usize(size.unwrap())?;

            return Aes::generate_key(bytes);
        },
        _ => {
            return Err(AuthenticationAppException::new(String::from("Symmetric module not found.")));
        }
    }
}

impl SymmetricKey {
    
    pub fn evalue(&self) -> Result<(), AuthenticationAppException::AuthenticationAppException> {
        let message = "message".as_bytes();

        let enc = self.encrypt_message(message);
        if enc.is_err() {
            return Err(AuthenticationAppException::new(enc.err().unwrap().to_string()));
        }

        let dec = self.decrypt_message(enc.unwrap().as_bytes());
        if dec.is_err() {
            return Err(AuthenticationAppException::new(dec.err().unwrap().to_string()));
        }

        if dec.unwrap().as_bytes() != message {
            return Err(AuthenticationAppException::new(String::from("Decoded data does not match with symetric encoded message content")));
        }

        return Ok(());
    }

    pub fn key(&self) -> Vec<u8> {
        return self.key.clone();
    }

    pub fn format(&self) -> String {
        return self.format.clone();
    }

    pub fn module(&self) -> String {
        return self.module.clone();
    }

    pub fn is_active(&self) -> bool {
        let current_system_time = SystemTime::now();
        let duration_since_epoch = current_system_time.duration_since(UNIX_EPOCH);
        let timestamp = duration_since_epoch.unwrap_or_default().as_millis();
        return self.status == SymmetricKeyState::ACTIVE && timestamp < self.timestamp + self.expires;
    }

    fn find_manager(&self) -> Result<impl SymmetricManager::SymmetricManager, AuthenticationApiException::AuthenticationApiException> {
        return Utils::find_manager(self.clone());
    }

    pub fn decrypt_message(&self, encrypted_message: &[u8]) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        let module = self.find_manager()?;
        return module.decrypt(encrypted_message);
    }
    
    pub fn encrypt_message(&self, message: &[u8]) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        let module = self.find_manager()?;
        return module.encrypt(message);
    }

}

#[derive(Clone, PartialEq)]
pub(crate) enum SymmetricKeyState {
    ACTIVE,
    EXPIRED
}

impl SymmetricKeyState {
    pub fn to_string(&self) -> &'static str {
        match self {
            SymmetricKeyState::ACTIVE => "ACTIVE",
            SymmetricKeyState::EXPIRED => "EXPIRED"
        }
    }
}