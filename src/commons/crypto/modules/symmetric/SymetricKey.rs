use std::time::{SystemTime, UNIX_EPOCH};

use crate::commons::crypto::modules::symmetric::Utils;
use crate::commons::exception::{AuthenticationApiException, AuthenticationAppException};

use crate::commons::crypto::modules::symmetric::AesBytes;

use super::{Aes, SymmetricManager};
use super::SymmetricManager::SymmetricManager as _;

#[derive(Clone)]
pub struct SymetricKey {
    module: String,
    key: Vec<u8>,
    format: String,
    expires: u128,
    timestamp: u128,
    status: SymetricKeyState
}

pub(crate) fn new(module: String, format: String, expires: u128) -> Result<SymetricKey, AuthenticationAppException::AuthenticationAppException> {
    let key = generate_key(module.clone(), format.clone());
    if key.is_err() {
        return Err(key.err().unwrap());
    }

    let current_system_time = SystemTime::now();
    let duration_since_epoch = current_system_time.duration_since(UNIX_EPOCH);
    let timestamp = duration_since_epoch.unwrap_or_default().as_millis();

    let data = SymetricKey {
        module: module,
        key: key.unwrap(),
        format: format,
        expires: expires,
        timestamp: timestamp,
        status: SymetricKeyState::ACTIVE
    };

    return Ok(data);
}

pub(crate) fn from(from: SymetricKey) -> Result<SymetricKey, AuthenticationAppException::AuthenticationAppException> {
    return new(from.module, from.format, from.expires);
}

fn generate_key(module: String, format: String) -> Result<Vec<u8>, AuthenticationAppException::AuthenticationAppException> {
    match module.as_str() {
        Aes::MODULE_CODE => {
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

impl SymetricKey {
    
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
        return self.status == SymetricKeyState::ACTIVE && timestamp < self.timestamp + self.expires;
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
pub(crate) enum SymetricKeyState {
    ACTIVE,
    EXPIRED
}

impl SymetricKeyState {
    pub fn to_string(&self) -> &'static str {
        match self {
            SymetricKeyState::ACTIVE => "ACTIVE",
            SymetricKeyState::EXPIRED => "EXPIRED"
        }
    }
}