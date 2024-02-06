use reqwest::StatusCode;

use crate::commons::exception::{AuthenticationApiException, AuthenticationAppException, ErrorCodes::ErrorCodes};

use super::SymmetricKey;

#[derive(Clone)]
pub struct SymmetricKeys {
    module: String,
    format: String,
    expires: u128,
    symetric_keys: Vec<SymmetricKey::SymmetricKey>
}

pub(crate) fn new(module: String, format: String, expires: u128) -> Result<SymmetricKeys, AuthenticationAppException::AuthenticationAppException> {
    let data = SymmetricKeys {
        module: module,
        format: format,
        expires: expires,
        symetric_keys: Vec::new()
    };

    return Ok(data);
}

impl SymmetricKeys {
    
    pub fn generate_new(&mut self) -> Result<SymmetricKey::SymmetricKey, AuthenticationAppException::AuthenticationAppException> {
        let symmetric_key = SymmetricKey::new(self.module.clone(), self.format.clone(), self.expires)?;
        self.symetric_keys.push(symmetric_key.clone());
        return Ok(symmetric_key);
    }

    //TODO: Remove.
    pub fn find(&mut self) -> Result<SymmetricKey::SymmetricKey, AuthenticationApiException::AuthenticationApiException> {
        let key = self.symetric_keys.iter().cloned().find(|k| k.is_active());
        if key.is_none() {
            let o_new_key = SymmetricKey::from(self.symetric_keys[0].clone());
            if o_new_key.is_err() {
                return Err(AuthenticationApiException::new(
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    ErrorCodes::SYSIN001,
                    o_new_key.err().unwrap().to_string()));
            }
            let new_key = o_new_key.unwrap();
            self.symetric_keys.push(new_key.clone());
            return Ok(new_key);
        }

        return Ok(key.unwrap());
    }

}