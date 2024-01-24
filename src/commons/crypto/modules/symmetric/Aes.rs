use aes::{cipher::{generic_array::GenericArray, ArrayLength, BlockDecrypt, BlockEncrypt, KeyInit}, Block};
use base64::{engine::general_purpose, Engine};
use rand::Rng;
use reqwest::StatusCode;

use crate::commons::exception::{AuthenticationApiException, AuthenticationAppException};

use crate::commons::crypto::modules::symmetric::{AesBytes, SymmetricManager::SymmetricManager};
use crate::commons::crypto::modules::symmetric::SymetricKey;

pub const MODULE_CODE: &str = "AES";

#[derive(Clone)]
pub struct Aes {
    key: Vec<u8>,
    bytes: usize,
}

pub(crate) fn new(bytes: AesBytes::AesBytes) -> Result<impl SymmetricManager, AuthenticationAppException::AuthenticationAppException> {    
    let key = generate_key(bytes.clone());
    if key.is_err() {
        return Err(key.err().unwrap());
    }

    let aes = Aes {
        key: key.unwrap(),
        bytes: bytes.as_usize(),
    };

    return Ok(aes);
}

pub(crate) fn from_symmetric(symmetric: SymetricKey::SymetricKey) -> Result<impl SymmetricManager, AuthenticationApiException::AuthenticationApiException> {    
    let size = symmetric.format().parse::<usize>();
    if size.is_err() {
        return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), size.err().unwrap().to_string()));
    }

    let bytes = AesBytes::from_usize(size.unwrap());
    if bytes.is_err() {
        return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), bytes.err().unwrap().to_string()));
    }

    let aes = Aes {
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
            let slice = self.clean_chunk(array.as_slice());
            let readable: String = String::from_utf8_lossy(&slice).into();
            buffer.push(readable.to_string());
        }

        let output = buffer.join("");
        return Ok(output);
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

    fn clean_chunk(&self, chunk: &[u8]) -> Vec<u8> {
        let mut chunc_copy = chunk.to_vec();
        let position = chunc_copy.iter().rev().position(|&byte| byte != b'\0');

        if let Some(index) = position {
            chunc_copy.truncate(chunk.len() - index);
        } else {
            chunc_copy.clear();
        }

        return chunc_copy;
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