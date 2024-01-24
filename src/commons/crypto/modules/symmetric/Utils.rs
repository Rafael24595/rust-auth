use reqwest::StatusCode;

use crate::commons::exception::AuthenticationApiException;

use super::{Aes, SymmetricManager};
use crate::commons::crypto::modules::symmetric::SymetricKey;

pub(crate) fn find_manager(symmetric: SymetricKey::SymetricKey) -> Result<impl SymmetricManager::SymmetricManager, AuthenticationApiException::AuthenticationApiException> {
    match symmetric.module().as_str() {
        Aes::MODULE_CODE => {
            let result = Aes::from_symmetric(symmetric);
            if result.is_err() {
                return Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), result.err().unwrap().to_string()));
            }
            return Ok(result.unwrap());
        }
        _ => {
            Err(AuthenticationApiException::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), String::from("Module not dound.")))
        }
    }
}