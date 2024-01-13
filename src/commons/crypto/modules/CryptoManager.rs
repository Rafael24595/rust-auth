use crate::commons::exception::AuthenticationApiException::AuthenticationApiException;

pub(crate) trait CryptoManager: Send + Clone {
    fn encrypt(&self, publ_string: String, message: &[u8]) -> Result<String, AuthenticationApiException>;
    fn decrypt(&self, priv_string: String, encrypted_message: &[u8]) -> Result<String, AuthenticationApiException>;
    fn sign(&self, priv_string: String, service: String) -> Result<String, AuthenticationApiException>;
    fn verify(&self, priv_string: String, service: String) -> Result<String, AuthenticationApiException>;
}