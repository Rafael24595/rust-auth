use aes::{cipher::{generic_array::GenericArray, ArrayLength, BlockDecrypt, BlockEncrypt, KeyInit}, Block};
use base64::{engine::general_purpose, Engine};
use rand::Rng;
use reqwest::StatusCode;

use crate::commons::exception::AuthenticationApiException;

use super::SymmetricManager::SymmetricManager;

pub const MODULE_CODE: &str = "AES";

#[derive(Clone)]
pub struct Aes {
    key: Vec<u8>,
    bytes: usize
}

pub(crate) fn new(bytes: usize) -> Result<impl SymmetricManager, AuthenticationApiException::AuthenticationApiException> {
    if bytes != 128 && bytes != 192 && bytes != 256 {
        return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), String::from("Bytes value must be 128, 192 or 256")));
    }
    
    let length = calculate_key_length(bytes);
    if length.is_err() {
        return Err(length.err().unwrap());
    }

    let key: Vec<u8> = (0..length.unwrap())
        .map(|_| rand::thread_rng().gen())
        .collect();

    let aes = Aes {
        key,
        bytes
    };

    return Ok(aes);
}

fn calculate_key_length(aes_bytes: usize) -> Result<usize, AuthenticationApiException::AuthenticationApiException> {
    let key_size = match aes_bytes {
        128 => 16,
        192 => 24,
        256 => 32,
        _ => {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), String::from("AES Bytes value must be 128, 192 or 256")));
        }
    };
    return Ok(key_size);
}

impl SymmetricManager for Aes {

    fn encrypt(&self, message: &[u8]) -> Result<String, AuthenticationApiException::AuthenticationApiException> {
        let o_cipher = self.build_cipher();
        if o_cipher.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), o_cipher.err().unwrap().to_string()));
        }

        let cipher = o_cipher.unwrap();

        let mut buffer = Vec::new();
        for chunk in message.chunks(16) {
            let cloned_chunk = self.fix_chunk(chunk);
            let mut array = *GenericArray::from_slice(&cloned_chunk);
            cipher.generic_encrypt_block(&mut array);
            buffer.push(array);
        }

        let b64 = self.encode_buffer(buffer);
        return Ok(b64);
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

        let mut buffer = Vec::new();
        for chunk in message_decoded.unwrap().chunks(16) {
            let cloned_chunk = self.fix_chunk(chunk);
            let mut array = *GenericArray::from_slice(&cloned_chunk);
            cipher.generic_decrypt_block(&mut array);
            let slice: &[u8] = array.as_slice();
            let readable: String = String::from_utf8_lossy(slice).into();
            buffer.push(readable.to_string());
        }

        let b64 = buffer.join("");
        return Ok(b64);
    }

}

impl Aes {

    fn build_cipher(&self) -> Result<Box<dyn AesGeneric> , AuthenticationApiException::AuthenticationApiException> {
        if self.bytes == 128 {
            let result = aes::Aes128::new_from_slice(&self.key);
            if result.is_ok() {
                return Ok(Box::new(result.unwrap()));
            }
        }
        if self.bytes == 192 {
            let result = aes::Aes192::new_from_slice(&self.key);
            if result.is_ok() {
                return Ok(Box::new(result.unwrap()));
            }
        }
        if self.bytes == 256 {
            let result = aes::Aes256::new_from_slice(&self.key);
            if result.is_ok() {
                return Ok(Box::new(result.unwrap()));
            }
        }
        return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), String::from("AES Bytes value must be 128, 192 or 256")));
    }

    fn encode_buffer<T: ArrayLength<u8>>(&self, buffer: Vec<GenericArray<u8, T>>) -> String {
        let bytes: Vec<u8>= buffer.iter()
            .map(|a| a.as_slice())
            .flatten()
            .cloned()
            .collect();

        return general_purpose::STANDARD.encode(&bytes);
    }
    
    fn fix_chunk(&self, chunk: &[u8]) -> [u8; 16] {
        let mut array = [0u8; 16];
        array[..chunk.len()].copy_from_slice(chunk);
        return array;
    }

}

trait AesGeneric {
    fn generic_encrypt_block(&self, block: &mut Block) ;
    fn generic_decrypt_block(&self, block: &mut Block) ;
}

impl AesGeneric for aes::Aes128 {
    fn generic_encrypt_block(&self, block: &mut Block) {
        self.encrypt_block(block);
    }
    fn generic_decrypt_block(&self, block: &mut Block) {
        self.decrypt_block(block);
    }
}

impl AesGeneric for aes::Aes192 {
    fn generic_encrypt_block(&self, block: &mut Block) {
        self.encrypt_block(block);
    }
    fn generic_decrypt_block(&self, block: &mut Block) {
        self.decrypt_block(block);
    }
}

impl AesGeneric for aes::Aes256 {
    fn generic_encrypt_block(&self, block: &mut Block) {
        self.encrypt_block(block);
    }
    fn generic_decrypt_block(&self, block: &mut Block) {
        self.decrypt_block(block);
    }
}