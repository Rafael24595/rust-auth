use crate::commons::exception::AuthenticationApiException::AuthenticationApiException;

pub(crate) trait CryptoManager: Send + Clone {
    fn encrypt(&self, priv_string: String, encrypted_message: &[u8]) -> Result<String, AuthenticationApiException>;
    fn decrypt(&self, priv_string: String, encrypted_message: &[u8]) -> Result<String, AuthenticationApiException>;
}