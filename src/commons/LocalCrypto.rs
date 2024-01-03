extern crate openssl;

use openssl::rsa::Rsa;
use openssl::symm::{Cipher, decrypt};

pub(crate) fn decrypt_message(encrypted_message: &[u8]) -> Result<String, openssl::error::ErrorStack> {
    Ok(String::new())
}

pub(crate) fn encrypt_message(encrypted_message: &[u8]) -> Result<String, openssl::error::ErrorStack> {
    Ok(String::new())
}