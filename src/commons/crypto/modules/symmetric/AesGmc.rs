use aes::{cipher::{generic_array::GenericArray, ArrayLength, BlockDecrypt, BlockEncrypt, KeyInit}, Block};
use aes_gcm::{aead::Aead, AeadCore, Aes128Gcm, Aes256Gcm, Key};
use base64::{engine::general_purpose, Engine};
use rand::Rng;
use reqwest::StatusCode;

use crate::commons::exception::{AuthenticationApiException, AuthenticationAppException};

use crate::commons::crypto::modules::symmetric::{AesBytes, SymmetricManager::SymmetricManager};
use crate::commons::crypto::modules::symmetric::SymmetricKey;

pub const MODULE_CODE: &str = "AES_GMC";

#[derive(Clone)]
pub struct AesGmc {
    key: Vec<u8>,
    bytes: usize,
}

pub(crate) fn new(bytes: AesBytes::AesBytes) -> Result<impl SymmetricManager, AuthenticationAppException::AuthenticationAppException> {    
    let key = generate_key(bytes.clone());
    if key.is_err() {
        return Err(key.err().unwrap());
    }

    let aes = AesGmc {
        key: key.unwrap(),
        bytes: bytes.as_usize(),
    };

    return Ok(aes);
}

pub(crate) fn from_symmetric(symmetric: SymmetricKey::SymmetricKey) -> Result<impl SymmetricManager, AuthenticationApiException::AuthenticationApiException> {    
    let size = symmetric.format().parse::<usize>();
    if size.is_err() {
        return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), size.err().unwrap().to_string()));
    }

    let bytes = AesBytes::from_usize(size.unwrap());
    if bytes.is_err() {
        return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), bytes.err().unwrap().to_string()));
    }

    let aes = AesGmc {
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
        192 => 24,
        256 => 32,
        _ => {
            return Err(AuthenticationAppException::new(String::from("AES Bytes value must be 128, 192 or 256")));
        }
    };
    return Ok(key_size);
}

impl SymmetricManager for AesGmc {

    fn encrypt(&self, message: &[u8]) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        let o_cipher = self.build_cipher();
        if o_cipher.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), o_cipher.err().unwrap().to_string()));
        }

        let cipher = o_cipher.unwrap();
        let result = cipher.generic_encrypt(message);
        if result.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), result.err().unwrap().to_string()));
        }

        return Ok(general_purpose::STANDARD.encode(&result.unwrap()));
    }

    fn decrypt(&self, encrypted_message: &[u8]) -> Result<String, crate::commons::exception::AuthenticationApiException::AuthenticationApiException> {
        let message_decoded = general_purpose::STANDARD.decode(encrypted_message);
        if message_decoded.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), message_decoded.err().unwrap().to_string()));
        }

        let o_cipher = self.build_cipher();
        if o_cipher.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), o_cipher.err().unwrap().to_string()));
        }

        let cipher = o_cipher.unwrap();
        let result = cipher.generic_encrypt(&message_decoded.unwrap());
        if result.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), result.err().unwrap().to_string()));
        }

        return Ok(String::from_utf8_lossy(&result.unwrap()).into());
    }

}

impl AesGmc {

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
        return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), String::from("AES Bytes value must be 128, 192 or 256")));
    }

}

trait AesGeneric {
    fn generic_encrypt(&self, message: &[u8]) -> Result<Vec<u8>, aes_gcm::Error>  ;
    fn generic_decrypt(&self, message: &[u8]) -> Result<Vec<u8>, aes_gcm::Error>  ;
}

impl AesGeneric for aes_gcm::Aes128Gcm {
    fn generic_encrypt(&self, message: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
        let iv = GenericArray::default();
        return self.encrypt(&iv, message);
    }
    fn generic_decrypt(&self, message: &[u8]) -> Result<Vec<u8>, aes_gcm::Error>  {
        let iv = GenericArray::default();
        return self.decrypt(&iv, message);
    }
}

impl AesGeneric for aes_gcm::Aes256Gcm {
    fn generic_encrypt(&self, message: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
        let iv = GenericArray::default();
        return self.encrypt(&iv, message);
    }
    fn generic_decrypt(&self, message: &[u8]) -> Result<Vec<u8>, aes_gcm::Error>  {
        let iv = GenericArray::default();
        return self.decrypt(&iv, message);
    }
}