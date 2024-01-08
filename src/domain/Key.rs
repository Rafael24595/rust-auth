use std::time::{SystemTime, UNIX_EPOCH};

use crate::infrastructure::{DtoPubKeyResponse, DtoPubKeyRequest};

#[derive(Clone, Debug)]
pub struct Key {
    key: String,
    module: String,
    format: String,
    passphrase: String,
    expires: u128
}

pub(crate) fn new(key: String, module: String, format: String, passphrase: String, expires: u128) -> Key {
    Key {
        key: key,
        module,
        format,
        passphrase,
        expires: expires
    }
}

pub(crate) fn from_dto(dto: DtoPubKeyRequest::DtoPubKeyRequest) -> Key {
    self::new(
        dto.key, 
        dto.module,
        dto.format,
        dto.passphrase, 
        dto.expires
    )
}

impl Key {
    
    pub fn key(&self) -> String {
        return self.key.to_string();
    }

    pub fn is_expired(&self) -> bool {
        let current_time = SystemTime::now();
        let duration_since_epoch = current_time.duration_since(UNIX_EPOCH).expect("Critical error.");
        let milliseconds = duration_since_epoch.as_millis();

        return self.expires < milliseconds;
    }

    pub fn as_dto(&self) -> DtoPubKeyResponse::DtoPubKeyResponse {
        return DtoPubKeyResponse::new(
            self.key.clone(), 
            self.module.clone(), 
            self.format.clone(), 
            self.passphrase.clone()
        );
    }
    
}