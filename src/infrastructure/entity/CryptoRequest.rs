use crate::infrastructure::entity::{HeaderParameter, QueryParameter};

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
    
    pub fn setMethod(&mut self, method: String) {
        self.method = method;
    }

    pub fn setService(&mut self, service: String) {
        self.service = service;
    }

    pub fn setQuery(&mut self, query: &str) {
        let query_fragments = query.split("&");
        for fragment in query_fragments {
            let e_query = QueryParameter::from(fragment);
            if e_query.is_some() {
                self.addQueryParameter(e_query.unwrap());
            }
        }
    }

    pub fn setPath(&mut self, path: String) {
        self.path = path;
    }

    pub fn setBody(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    pub fn addQueryParameter(&mut self, query: QueryParameter::QueryParameter) -> Option<QueryParameter::QueryParameter> {
        let mut original = None;
        let index = self.query_params.iter().position(|q| q.key() == query.key());
        if index.is_some() {
            original = Some(self.query_params.remove(index.unwrap()));
        }
        self.query_params.push(query);
        return original;
    }

    pub fn addHeaderParameter(&mut self, header: HeaderParameter::HeaderParameter) -> HeaderParameter::HeaderParameter {
        let index = self.headers.iter().position(|h| h.key() == header.key());
        if index.is_some() {
            let header_base = self.headers.get_mut(index.unwrap()).unwrap();
            header_base.addValues(header.values());
            return header_base.clone();
        }
        self.headers.push(header.clone());
        return header;
    }

    pub fn addHeaderParameterTuple(&mut self, key: String, value: String) -> HeaderParameter::HeaderParameter {
        let header = HeaderParameter::new(key, value);
        return self.addHeaderParameter(header);
    }

}