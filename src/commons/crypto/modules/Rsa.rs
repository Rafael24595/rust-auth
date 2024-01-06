use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePrivateKey};

use crate::commons::crypto::modules::CryptoManager;

pub const MODULE_CODE: &str = "RSA";

#[derive(Clone)]
pub struct Rsa {
    format: String,
    is_pem: bool,
    pass_phrase: String
}

pub(crate) fn new(format: String, pass_phrase: String) -> impl CryptoManager::CryptoManager {
    let format_fragments: Vec<&str> = format.split("&").collect();
    let format_value = format_fragments.first().unwrap().to_string();
    let is_pem = format_fragments.last().unwrap().to_string().eq("PEM");

    Rsa {
        format: format_value,
        is_pem: is_pem,
        pass_phrase: pass_phrase
    }
}

impl CryptoManager::CryptoManager for Rsa {
    
    fn encrypt(&self, priv_string: String, encrypted_message: &[u8]) -> Result<String, String> {
        Ok(String::new())
    }

    fn decrypt(&self, priv_string: String, encrypted_message: &[u8]) -> Result<String, String> {
        let priv_key = RsaPrivateKey::from_pkcs1_pem(&priv_string);
    
        if priv_key.is_err() {
            println!("{}", priv_key.clone().err().unwrap());
        }
    
        let dec_data = priv_key.unwrap().decrypt(Pkcs1v15Encrypt, &encrypted_message).expect("failed to decrypt");
        let string = String::from_utf8(dec_data);
    
        Ok(string.unwrap())
    }

}