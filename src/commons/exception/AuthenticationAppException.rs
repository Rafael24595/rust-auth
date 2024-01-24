use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct AuthenticationAppException {
    message: String,
}

impl fmt::Display for AuthenticationAppException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomError: {}", self.message)
    }
}

impl Error for AuthenticationAppException {}

pub(crate) fn new(message: String) -> AuthenticationAppException {
    return AuthenticationAppException {
        message
    };
}

impl AuthenticationAppException {
    
    pub fn message(&self) -> String {
        return self.message.clone();
    }

}