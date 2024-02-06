use aes_gcm::KeyInit;
use aes_gcm::{aead::Aead, Aes128Gcm, Aes256Gcm, Key, Nonce};
use base64::{engine::general_purpose, Engine};
use rand::Rng;
use reqwest::StatusCode;

use crate::commons::exception::ErrorCodes::ErrorCodes;
use crate::commons::exception::{AuthenticationApiException, AuthenticationAppException};

use crate::commons::crypto::modules::symmetric::{AesBytes, SymmetricManager::SymmetricManager};
use crate::commons::crypto::modules::symmetric::SymmetricKey;

use super::AesGcmMessage;

pub const MODULE_CODE: &str = "AES_GCM";

#[derive(Clone)]
pub struct AesGcm {
    key: Vec<u8>,
    bytes: usize,
}

pub(crate) fn new(bytes: AesBytes::AesBytes) -> Result<impl SymmetricManager, AuthenticationAppException::AuthenticationAppException> {    
    let key = generate_key(bytes.clone());
    if key.is_err() {
        return Err(key.err().unwrap());
    }

    let aes = AesGcm {
        key: key.unwrap(),
        bytes: bytes.as_usize(),
    };

    return Ok(aes);
}

pub(crate) fn from_symmetric(symmetric: SymmetricKey::SymmetricKey) -> Result<impl SymmetricManager, AuthenticationApiException::AuthenticationApiException> {    
    let size = symmetric.format().parse::<usize>();
    if size.is_err() {
        return Err(AuthenticationApiException::new(
            StatusCode::NOT_ACCEPTABLE.as_u16(),
            ErrorCodes::CLIUA004,
            size.err().unwrap().to_string()));
    }

    let bytes = AesBytes::from_usize(size.unwrap());
    if bytes.is_err() {
        return Err(AuthenticationApiException::new(
            StatusCode::NOT_ACCEPTABLE.as_u16(),
            ErrorCodes::CLIUA004,
            bytes.err().unwrap().to_string()));
    }

    let aes = AesGcm {
        key: symmetric.key(),
        bytes: bytes.unwrap().as_usize(),
    };

    return Ok(aes);
}

pub(crate) fn generate_key(bytes: AesBytes::AesBytes) -> Result<Vec<u8>, AuthenticationAppException::AuthenticationAppException> {    
    let length = calculate_key_length(bytes.as_usize());
    if length.is_err() {
        return Err(length.err().unwrap());
    }

    let key: Vec<u8> = (0..length.unwrap())
        .map(|_| rand::thread_rng().gen())
        .collect();

    return Ok(key);
}

fn calculate_key_length(aes_bytes: usize) -> Result<usize, AuthenticationAppException::AuthenticationAppException> {
    let key_size = match aes_bytes {
        128 => 16,
        256 => 32,
        _ => {
            return Err(AuthenticationAppException::new(String::from("AES Bytes value must be 128, 192 or 256")));
        }
    };
    return Ok(key_size);
}

impl SymmetricManager for AesGcm {

    fn encrypt(&self, message: &[u8]) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        let cipher = self.build_cipher()?;
        let result = cipher.generic_encrypt(message);
        if result.is_err() {
            return Err(AuthenticationApiException::new(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                ErrorCodes::CLIFB008,
                result.err().unwrap().to_string()));
        }
        return Ok(result.unwrap().to_string());
    }

    fn decrypt(&self, encrypted_message: &[u8]) -> Result<String, crate::commons::exception::AuthenticationApiException::AuthenticationApiException> {
        let message = AesGcmMessage::from_slice(encrypted_message);
        if message.is_err() {
            return Err(message.err().unwrap());
        }

        let cipher = self.build_cipher()?;
        let result = cipher.generic_decrypt(message.unwrap());
        if result.is_err() {
            return Err(AuthenticationApiException::new(
                StatusCode::FORBIDDEN.as_u16(),
                ErrorCodes::CLIFB007,
                result.err().unwrap().to_string()));
        }

        return Ok(String::from_utf8_lossy(&result.unwrap()).into());
    }

}

impl AesGcm {

    fn build_cipher(&self) -> Result<Box<dyn AesGeneric> , AuthenticationApiException::AuthenticationApiException> {
        if self.bytes == 128 {
            let key = Key::<Aes128Gcm>::from_slice(&self.key);
            let cipher = Aes128Gcm::new(&key);
            return Ok(Box::new(cipher));
        }
        if self.bytes == 256 {
            let key = Key::<Aes256Gcm>::from_slice(&self.key);
            let cipher = Aes256Gcm::new(&key);
            return Ok(Box::new(cipher));
        }
        return Err(AuthenticationApiException::new(
            StatusCode::NOT_ACCEPTABLE.as_u16(),
            ErrorCodes::CLIUA004,
            String::from("AES Bytes value must be 128 or 256")));
    }

}

trait AesGeneric {
    fn generic_encrypt(&self, message: &[u8]) -> Result<AesGcmMessage::AesGcmMessage, aes_gcm::Error>  ;
    fn generic_decrypt(&self, message: AesGcmMessage::AesGcmMessage) -> Result<Vec<u8>, aes_gcm::Error>  ;
}

impl AesGeneric for aes_gcm::Aes128Gcm {
    fn generic_encrypt(&self, message: &[u8]) -> Result<AesGcmMessage::AesGcmMessage, aes_gcm::Error> {
        let iv: Vec<u8> = (0..12)
            .map(|_| rand::thread_rng().gen())
            .collect();

        let nonce = Nonce::from_slice(&iv);
        let payload = self.encrypt(&nonce, message)?;

        let nonce64 = general_purpose::STANDARD.encode(nonce);
        let payload64 = general_purpose::STANDARD.encode(payload);

        return Ok(AesGcmMessage::new(
            nonce64.as_bytes().to_vec(),
            payload64.as_bytes().to_vec()
        ));
    }
    fn generic_decrypt(&self, message: AesGcmMessage::AesGcmMessage) -> Result<Vec<u8>, aes_gcm::Error>  {
        let binding = message.nonce();
        let nonce = Nonce::from_slice(&binding);
        let payload: &[u8] = &message.payload();
        return self.decrypt(&nonce, payload);
    }
}

impl AesGeneric for aes_gcm::Aes256Gcm {
    fn generic_encrypt(&self, message: &[u8]) -> Result<AesGcmMessage::AesGcmMessage, aes_gcm::Error> {
        let iv: Vec<u8> = (0..12)
            .map(|_| rand::thread_rng().gen())
            .collect();

        let nonce = Nonce::from_slice(&iv);
        let payload = self.encrypt(&nonce, message)?;

        let nonce64 = general_purpose::STANDARD.encode(nonce);
        let payload64 = general_purpose::STANDARD.encode(payload);

        return Ok(AesGcmMessage::new(
            nonce64.as_bytes().to_vec(),
            payload64.as_bytes().to_vec()
        ));
    }
    fn generic_decrypt(&self, message: AesGcmMessage::AesGcmMessage) -> Result<Vec<u8>, aes_gcm::Error>  {
        let binding = message.nonce();
        let nonce = Nonce::from_slice(&binding);
        let payload: &[u8] = &message.payload();
        return self.decrypt(&nonce, payload);
    }
}