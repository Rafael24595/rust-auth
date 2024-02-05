use reqwest::StatusCode;

use crate::commons::exception::{AuthenticationApiException, ErrorCodes::ErrorCodes};

use super::{AesGcm, SymmetricManager};
use crate::commons::crypto::modules::symmetric::SymmetricKey;

pub(crate) fn find_manager(symmetric: SymmetricKey::SymmetricKey) -> Result<impl SymmetricManager::SymmetricManager, AuthenticationApiException::AuthenticationApiException> {
    match symmetric.module().as_str() {
        AesGcm::MODULE_CODE => {
            let result = AesGcm::from_symmetric(symmetric)?;
            return Ok(result);
        }
        _ => {
            Err(AuthenticationApiException::new(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                ErrorCodes::SYSIN001,
                String::from("Module not found.")))
        }
    }
}