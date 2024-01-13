use std::fs::File;
use std::io::Read;
use reqwest::StatusCode;

use crate::commons::crypto::modules::{CryptoManager, Rsa};
use crate::commons::crypto::modules::CryptoManager::CryptoManager as _;
use crate::commons::exception::AuthenticationApiException;

#[derive(Clone)]
pub struct CryptoConfiguration {
    pubkey_name: String,
    prikey_name: String,
    module: String,
    format: String,
    pass_phrase: String
}

pub(crate) fn new(pubkey_name: String, prikey_name: String, module: String, format: String, pass_phrase: String) -> CryptoConfiguration {
    CryptoConfiguration {
        pubkey_name,
        prikey_name,
        module,
        format,
        pass_phrase
    }
}

impl CryptoConfiguration {

    pub fn module(&self) -> String {
        return self.module.clone();
    }
    
    pub fn format(&self) -> String {
        return self.format.clone();
    }

    pub fn pass_phrase(&self) -> String {
        return self.pass_phrase.clone();
    }

    fn find_manager(&self) -> Result<impl CryptoManager::CryptoManager, String> {
        match self.module.as_str() {
            Rsa::MODULE_CODE => {
                return Ok(Rsa::new(self.format.clone(), self.pass_phrase.clone()));
            }
            _ => {
                Err(String::from("Module not dound."))
            }
        }
    }

    pub fn decrypt_message(&self, encrypted_message: &[u8]) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        let priv_string = self.read_private();
        if priv_string.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), priv_string.err().unwrap().to_string()));
        }

        let module = self.find_manager();
        if module.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), module.err().unwrap().to_string()));
        }

        return module.unwrap().decrypt(priv_string.unwrap(), encrypted_message);
    }
    
    pub fn encrypt_message(&self, encrypted_message: &[u8]) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        Ok(String::new())
    }

    pub fn sign(&self, message: String) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        let priv_string = self.read_private();
        if priv_string.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), priv_string.err().unwrap().to_string()));
        }

        let module = self.find_manager();
        if module.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), module.err().unwrap().to_string()));
        }

        return module.unwrap().sign(priv_string.unwrap(), message);
    }

    pub fn verify(&self, message: String) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        let priv_string = self.read_private();
        if priv_string.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), priv_string.err().unwrap().to_string()));
        }

        let module = self.find_manager();
        if module.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), module.err().unwrap().to_string()));
        }

        return module.unwrap().verify(priv_string.unwrap(), message);
    }

    fn read_private(&self) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        return self.read_key(self.prikey_name.clone());
    }
    
    pub fn read_public(&self) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        return self.read_key(self.pubkey_name.clone());
    }
    
    fn read_key(&self, name: String) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        let file_path = String::from("./assets/keys/") + &name;
        let file = File::open(file_path);
    
        if file.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), file.err().unwrap().to_string()));
        }
    
        let mut key = String::new();
        let result = file.unwrap().read_to_string(&mut key);
    
        if result.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), result.err().unwrap().to_string()));
        }

        let key_clean = key
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join("\n");
    
        return Ok(key_clean);
    }

}