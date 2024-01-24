use reqwest::StatusCode;

use crate::commons::crypto::modules::asymmetric::AsymmetricManager;
use crate::commons::crypto::ServiceToken;
use crate::commons::exception::AuthenticationApiException;

use super::AsymmetricManager::AsymmetricManager as _;
use super::Utils;

#[derive(Clone)]
pub struct AsymmetricPrivate {
    prikey: String,
    module: String,
    format: String,
    pass_phrase: String,
    expires_range: u128
}

pub(crate) fn new(prikey: String, module: String, format: String, pass_phrase: String, expires_range: u128) -> AsymmetricPrivate {
    AsymmetricPrivate {
        prikey,
        module,
        format,
        pass_phrase,
        expires_range
    }
}

impl AsymmetricPrivate {

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

    pub fn decrypt_message(&self, encrypted_message: &[u8]) -> Result<Vec<u8>, AuthenticationApiException::AuthenticationApiException> {
        let module = self.find_manager();
        if module.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), module.err().unwrap().to_string()));
        }

        return module.unwrap().decrypt(self.prikey.clone(), encrypted_message);
    }

    pub fn sign(&self, message: String) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        let module = self.find_manager();
        if module.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), module.err().unwrap().to_string()));
        }

        let r_token = module.unwrap().sign(self.prikey.clone(), message, self.expires_range);
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
        let module = self.find_manager();
        if module.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), module.err().unwrap().to_string()));
        }

        let r_token = ServiceToken::from_string(message);
        if r_token.is_err() {
            return Err(r_token.err().unwrap());
        }

        let token = r_token.unwrap();

        let lifetime_validation = token.is_alive();
        if lifetime_validation.is_err() && !lifetime_validation.clone().err().unwrap().0 {
            return Err(lifetime_validation.err().unwrap().1);
        }

        let result = module.unwrap().verify(self.prikey.clone(), token.clone());
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        let mut refresh = None;
        if lifetime_validation.is_err() && lifetime_validation.err().unwrap().0 {
            let token_refresh = self.refresh(token);
            if token_refresh.is_ok() {
                refresh = Some(token_refresh.unwrap());
            }
            //TODO: Log.
        }

        return Ok(refresh);
    }

}