use base64::{engine::general_purpose, Engine};
use sha2::digest::{Update, FixedOutput};

use crate::{infrastructure::entity::{HeaderParameter, QueryParameter}, commons::configuration::Configuration};

#[derive(Clone, Debug)]
pub struct CryptoRequest {
    method: String,
    service: String,
    path: String,
    query_params: Vec<QueryParameter::QueryParameter>,
    headers: Vec<HeaderParameter::HeaderParameter>,
    body: Vec<u8>,
}

pub(crate) fn new() -> CryptoRequest {
    CryptoRequest {
        method: String::from("GET"),
        service: String::new(),
        path: String::new(),
        query_params: Vec::new(),
        headers: Vec::new(),
        body: Vec::from(String::new())
    }
}

impl CryptoRequest {
    
    pub fn method(&self) -> String {
        return self.method.clone();
    }

    pub fn service(&self) -> String {
        return self.service.clone();
    }

    pub fn uri(&self) -> String {
        let mut query = self.query();
        if !query.is_empty() {
            query = String::from("?") + &query;
        }
        return self.path.clone() + &query;
    }

    pub fn query(&self) -> String {
        let query: Vec<String> = self.query_params.iter()
            .map(|p| p.key() + "=" + &p.value())
            .collect();
        return query.join("&");
    }

    pub fn body(&self) -> Vec<u8> {
        return self.body.clone();
    }

    pub fn headers(&self) -> Vec<HeaderParameter::HeaderParameter> {
        return self.headers.clone();
    }

    pub fn set_method(&mut self, method: String) {
        self.method = method;
    }

    pub fn set_service(&mut self, service: String) {
        self.service = service;
    }

    pub fn set_query(&mut self, query: &str) {
        let query_fragments = query.split("&");
        for fragment in query_fragments {
            let e_query = QueryParameter::from(fragment);
            if e_query.is_some() {
                self.add_query_parameter(e_query.unwrap());
            }
        }
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
        self.set_integrity_header();
    }

    fn set_integrity_header(&mut self) {
        let mut hash_raw: sha2::Sha256 = sha2::Digest::new();
        hash_raw.update(&self.body.clone());
        let result = hash_raw.finalize_fixed().to_vec();
        let hash_64 = general_purpose::STANDARD.encode(result);
        self.set_header_parameter_tuple(Configuration::HEADER_INTEGRITY_NAME.to_string(), String::from("SHA256;") + &hash_64);
    }

    pub fn add_query_parameter(&mut self, query: QueryParameter::QueryParameter) -> Option<QueryParameter::QueryParameter> {
        let mut original = None;
        let index = self.query_params.iter().position(|q| q.key() == query.key());
        if index.is_some() {
            original = Some(self.query_params.remove(index.unwrap()));
        }
        self.query_params.push(query);
        return original;
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