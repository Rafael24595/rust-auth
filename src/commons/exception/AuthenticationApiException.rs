use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct AuthenticationApiException {
    status: u16,
    message: String,
}

impl fmt::Display for AuthenticationApiException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomError: {}", self.message)
    }
}

impl Error for AuthenticationApiException {}

pub(crate) fn new(status: u16, message: String) -> AuthenticationApiException {
    return AuthenticationApiException {
        status,
        message
    };
}

impl AuthenticationApiException {

    pub fn status(&self) -> u16 {
        return self.status;
    }
    
    pub fn message(&self) -> String {
        return self.message.clone();
    }

}