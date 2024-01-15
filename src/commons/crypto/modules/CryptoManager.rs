use crate::commons::{exception::AuthenticationApiException::AuthenticationApiException, crypto::ServiceToken};

pub(crate) trait CryptoManager: Send + Clone {
    fn encrypt(&self, publ_string: String, message: &[u8]) -> Result<Vec<u8>, AuthenticationApiException>;
    fn decrypt(&self, priv_string: String, encrypted_message: &[u8]) -> Result<Vec<u8>, AuthenticationApiException>;
    fn sign(&self, priv_string: String, service: String, expires_range: u128) -> Result<ServiceToken::ServiceToken, AuthenticationApiException>;
    fn verify(&self, priv_string: String, token: ServiceToken::ServiceToken) -> Result<String, AuthenticationApiException>;
}