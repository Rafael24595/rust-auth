use reqwest::StatusCode;

use crate::commons::exception::{AuthenticationApiException, AuthenticationAppException};

use super::SymetricKey;

#[derive(Clone)]
pub struct SymetricKeys {
    module: String,
    format: String,
    expires: u128,
    symetric_keys: Vec<SymetricKey::SymetricKey>
}

pub(crate) fn new(module: String, format: String, expires: u128) -> Result<SymetricKeys, AuthenticationAppException::AuthenticationAppException> {
    let data = SymetricKeys {
        module: module,
        format: format,
        expires: expires,
        symetric_keys: Vec::new()
    };

    return Ok(data);
}

impl SymetricKeys {
    
    pub fn generate_new(&mut self) -> Result<SymetricKey::SymetricKey, AuthenticationAppException::AuthenticationAppException> {
        let symmetric_key = SymetricKey::new(self.module.clone(), self.format.clone(), self.expires)?;
        self.symetric_keys.push(symmetric_key.clone());
        return Ok(symmetric_key);
    }

    pub fn find(&mut self) -> Result<SymetricKey::SymetricKey, AuthenticationApiException::AuthenticationApiException> {
        let key = self.symetric_keys.iter().cloned().find(|k| k.is_active());
        if key.is_none() {
            let o_new_key = SymetricKey::from(self.symetric_keys[0].clone());
            if o_new_key.is_err() {
                return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), o_new_key.err().unwrap().to_string()));
            }
            let new_key = o_new_key.unwrap();
            self.symetric_keys.push(new_key.clone());
            return Ok(new_key);
        }

        return Ok(key.unwrap());
    }

}