use crate::commons::crypto::ServiceToken;
use crate::commons::exception::{AuthenticationApiException, AuthenticationAppException};
use crate::infrastructure::DtoPubKeyResponse;

use super::{AsymmetricPrivate, AsymmetricPublic};

#[derive(Clone)]
pub struct AsymmetricKeys {
    pubkey: AsymmetricPublic::AsymmetricPublic,
    prikey: AsymmetricPrivate::AsymmetricPrivate,
}

pub(crate) fn new(pubkey: AsymmetricPublic::AsymmetricPublic, prikey: AsymmetricPrivate::AsymmetricPrivate) -> AsymmetricKeys {
    AsymmetricKeys {
        pubkey,
        prikey
    }
}

impl AsymmetricKeys {

    pub fn evalue(&self) -> Result<(), AuthenticationAppException::AuthenticationAppException> {
        let message = "message".as_bytes();

        let enc = self.encrypt_message(message);
        if enc.is_err() {
            return Err(AuthenticationAppException::new(enc.err().unwrap().to_string()));
        }

        let dec = self.decrypt_message(&enc.unwrap());
        if dec.is_err() {
            return Err(AuthenticationAppException::new(dec.err().unwrap().to_string()));
        }

        if dec.unwrap().as_slice() != message {
            return Err(AuthenticationAppException::new(String::from("Decoded data does not match with asymetric encoded message content")));
        }

        return Ok(());
    }

    pub fn public_key(&self) -> DtoPubKeyResponse::DtoPubKeyResponse {
        return self.pubkey.as_dto();
    }

    pub fn decrypt_message(&self, encrypted_message: &[u8]) -> Result<Vec<u8>, AuthenticationApiException::AuthenticationApiException> {
        return self.prikey.decrypt_message(encrypted_message);
    }
    
    pub fn encrypt_message(&self, message: &[u8]) -> Result<Vec<u8>, AuthenticationApiException::AuthenticationApiException> {
        return self.pubkey.encrypt_message(message);
    }

    pub fn sign(&self, message: String) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        return self.prikey.sign(message);
    }

    pub fn refresh(&self, token: ServiceToken::ServiceToken) -> Result<ServiceToken::ServiceToken, AuthenticationApiException::AuthenticationApiException> {
        return self.prikey.refresh(token);
    }

    pub fn verify(&self, message: String) -> Result<Option<ServiceToken::ServiceToken>, AuthenticationApiException::AuthenticationApiException> {
        return self.prikey.verify(message);
    }

}