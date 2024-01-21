use base64::{engine::general_purpose, Engine};
use sha2::digest::{Update, FixedOutput};

use crate::{infrastructure::entity::HeaderParameter, commons::configuration::Configuration};

#[derive(Clone, Debug)]
pub struct CryptoResponse {
    status: u16,
    headers: Vec<HeaderParameter::HeaderParameter>,
    body: Vec<u8>,
}

pub(crate) fn new() -> CryptoResponse {
    CryptoResponse {
        status: 200,
        headers: Vec::new(),
        body: Vec::from(String::new())
    }
}

impl CryptoResponse {
    
    pub fn is_success(&self) -> bool {
        return 300 > self.status && self.status >= 200
    }

    pub fn status(&self) -> u16 {
        return self.status;
    }

    pub fn headers(&self) -> Vec<HeaderParameter::HeaderParameter> {
        return self.headers.clone();
    }

    pub fn body(&self) -> Vec<u8> {
        return self.body.clone();
    }

    pub fn set_body (&mut self, body: Vec<u8>) {
        self.body = body;
        self.set_integrity_header();
    }

    fn set_integrity_header(&mut self) {
        let mut hash_raw: sha2::Sha256 = sha2::Digest::new();
        hash_raw.update(&self.body.clone());
        let result = hash_raw.finalize_fixed().to_vec();
        let hash_64 = general_purpose::STANDARD.encode(result);
        self.set_header_parameter_tuple(Configuration::HEADER_INTEGRITY_NAME.to_string(), hash_64);
    }

    pub fn set_status (&mut self, status: u16) {
        self.status = status;
    }

    pub fn add_header_parameter(&mut self, header: HeaderParameter::HeaderParameter) -> HeaderParameter::HeaderParameter {
        let index = self.headers.iter().position(|h| h.key() == header.key());
        if index.is_some() {
            let header_base = self.headers.get_mut(index.unwrap()).unwrap();
            header_base.addValues(header.values());
            return header_base.clone();
        }
        self.headers.push(header.clone());
        return header;
    }

    pub fn set_header_parameter(&mut self, header: HeaderParameter::HeaderParameter) -> HeaderParameter::HeaderParameter {
        let index = self.headers.iter().position(|h| h.key() == header.key());
        if index.is_some() {
            self.headers.remove(index.unwrap());
        }
        self.headers.push(header.clone());
        return header;
    }

    pub fn add_header_parameter_tuple(&mut self, key: String, value: String) -> HeaderParameter::HeaderParameter {
        let header = HeaderParameter::new(key, value);
        return self.add_header_parameter(header);
    }

    pub fn set_header_parameter_tuple(&mut self, key: String, value: String) -> HeaderParameter::HeaderParameter {
        let header = HeaderParameter::new(key, value);
        return self.set_header_parameter(header);
    }

}