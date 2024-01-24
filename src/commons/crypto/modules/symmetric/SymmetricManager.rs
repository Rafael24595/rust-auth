use crate::commons::exception::AuthenticationApiException::AuthenticationApiException;

pub(crate) trait SymmetricManager: Send + Clone {
    fn encrypt(&self, message: &[u8]) -> Result<String, AuthenticationApiException>;
    fn decrypt(&self, encrypted_message: &[u8]) -> Result<String, AuthenticationApiException>;
}