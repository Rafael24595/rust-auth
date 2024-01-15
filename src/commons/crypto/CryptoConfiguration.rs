use std::fs::File;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::StatusCode;

use crate::commons::crypto::modules::{CryptoManager, Rsa};
use crate::commons::crypto::modules::CryptoManager::CryptoManager as _;
use crate::commons::exception::AuthenticationApiException;

use super::ServiceToken;

#[derive(Clone)]
pub struct CryptoConfiguration {
    pubkey_name: String,
    prikey_name: String,
    module: String,
    format: String,
    pass_phrase: String,
    expires_range: u128
}

pub(crate) fn new(pubkey_name: String, prikey_name: String, module: String, format: String, pass_phrase: String, expires_range: u128) -> CryptoConfiguration {
    CryptoConfiguration {
        pubkey_name,
        prikey_name,
        module,
        format,
        pass_phrase,
        expires_range
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

    pub fn decrypt_message(&self, encrypted_message: &[u8]) -> Result<Vec<u8>, AuthenticationApiException::AuthenticationApiException> {
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
    
    pub fn encrypt_message(&self, message: &[u8]) -> Result<Vec<u8>, AuthenticationApiException::AuthenticationApiException> {
        let publ_string = self.read_public();
        if publ_string.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), publ_string.err().unwrap().to_string()));
        }

        let module = self.find_manager();
        if module.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), module.err().unwrap().to_string()));
        }

        return module.unwrap().encrypt(publ_string.unwrap(), message);
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

        let r_token = module.unwrap().sign(priv_string.unwrap(), message, self.expires_range);
        if r_token.is_err() {
            return Err(r_token.err().unwrap());
        }

        return Ok(r_token.unwrap().to_string());
    }

    pub fn refresh(&self, token: ServiceToken::ServiceToken) -> Result<ServiceToken::ServiceToken, AuthenticationApiException::AuthenticationApiException> {
        let s_token =  self.sign(token.payload().service);
        if s_token.is_err() {
            return Err(s_token.err().unwrap());
        }
        return ServiceToken::from_string(s_token.unwrap());
    }

    pub fn verify(&self, message: String) -> Result<Option<ServiceToken::ServiceToken>, AuthenticationApiException::AuthenticationApiException> {
        let priv_string = self.read_private();
        if priv_string.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), priv_string.err().unwrap().to_string()));
        }

        let module = self.find_manager();
        if module.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), priv_string.err().unwrap().to_string()));
        }

        let token = ServiceToken::from_string(message);
        if token.is_err() {
            return Err(token.err().unwrap());
        }

        let lifetime_validation = self.verify_lifetime(token.clone().unwrap());
        if lifetime_validation.is_err() && !lifetime_validation.clone().err().unwrap().0 {
            return Err(lifetime_validation.err().unwrap().1);
        }

        let result = module.unwrap().verify(priv_string.unwrap(), token.clone().unwrap());
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        let mut refresh = None;
        if lifetime_validation.err().unwrap().0 {
            let token_refresh = self.refresh(token.unwrap());
            if token_refresh.is_ok() {
                refresh = Some(token_refresh.unwrap());
            }
            //TODO: Log.
        }

        return Ok(refresh);
    }

    fn verify_lifetime(&self, token: ServiceToken::ServiceToken) -> Result<(), (bool, AuthenticationApiException::AuthenticationApiException)> {
        let current_system_time = SystemTime::now();
        let duration_since_epoch = current_system_time.duration_since(UNIX_EPOCH);
        let timestamp = duration_since_epoch.unwrap_or_default().as_millis();

        if timestamp > token.payload().expires {
            let refresh = (timestamp - token.payload().expires) < 240000;
            return Err((refresh, AuthenticationApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Token has expired."))));
        }

        return Ok(());
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