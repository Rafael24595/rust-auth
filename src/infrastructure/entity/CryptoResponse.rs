use crate::infrastructure::entity::HeaderParameter;

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

    pub fn add_header_parameter_tuple(&mut self, key: String, value: String) -> HeaderParameter::HeaderParameter {
        let header = HeaderParameter::new(key, value);
        return self.add_header_parameter(header);
    }

}