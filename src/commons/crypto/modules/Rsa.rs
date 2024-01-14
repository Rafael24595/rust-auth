use rsa::pkcs1v15;
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePrivateKey};
use reqwest::StatusCode;

use rsa::pkcs1v15::SigningKey;
use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
use rsa::sha2::Sha256;

use crate::commons::crypto::{Payload, ServiceToken};
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

    fn sign(&self, priv_string: String, service: String) -> Result<ServiceToken::ServiceToken, AuthenticationApiException::AuthenticationApiException> {
        let o_priv_key = self.private_key(priv_string);
        if o_priv_key.is_err() {
            return Err(o_priv_key.err().unwrap());
        }

        let priv_key = o_priv_key.unwrap();

        let mut rng = rand::thread_rng();
        let signing_key : SigningKey<Sha256> = pkcs1v15::SigningKey::new(priv_key);
        let signature = signing_key.sign_with_rng(&mut rng, service.as_bytes());

        let payload = Payload::new(service);
        let token = ServiceToken::new(signature.to_bytes().to_vec(), payload);

        return Ok(token);
    }

    fn verify(&self, priv_string: String, token: ServiceToken::ServiceToken) -> Result<(), AuthenticationApiException::AuthenticationApiException> {
        let o_priv_key = self.private_key(priv_string);
        if o_priv_key.is_err() {
            return Err(o_priv_key.err().unwrap());
        }

        let priv_key = o_priv_key.unwrap();
        let signing_key: SigningKey<Sha256> = pkcs1v15::SigningKey::new(priv_key);
        let verifying_key = signing_key.verifying_key();
        
        let sign: &[u8] = &token.sign();
        let payload = token.payload();

        let signature = pkcs1v15::Signature::try_from(sign);
        if signature.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Malformed token.")));
        }

        let result = verifying_key.verify(payload.service.as_bytes(), &signature.unwrap());
        if result.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Unautorized.")));
        }

        return Ok(());
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

    fn public_key(&self, publ_string: String) -> Result<RsaPublicKey, AuthenticationApiException::AuthenticationApiException> {
        if self.is_pem {
            let publ_key = RsaPublicKey::from_pkcs1_pem(&publ_string);
                if publ_key.is_err() {
                    return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), publ_key.err().unwrap().to_string()));
                }
                return Ok(publ_key.unwrap());
        }

        return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), String::from("Invalid format")));
    }

}