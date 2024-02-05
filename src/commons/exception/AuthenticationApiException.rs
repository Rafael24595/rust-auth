use std::fmt;
use std::error::Error;

use crate::commons::exception::ErrorCodes;

pub(crate) const EXCEPTION_HEADER: &str = "Error-Code";

#[derive(Debug, Clone)]
pub struct AuthenticationApiException {
    status: u16,
    error_code: ErrorCodes::ErrorCodes,
    message: String,
}

impl fmt::Display for AuthenticationApiException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomError: {}", self.message)
    }
}

impl Error for AuthenticationApiException {}

pub(crate) fn new(status: u16, error_code: ErrorCodes::ErrorCodes, message: String) -> AuthenticationApiException {
    return AuthenticationApiException {
        status,
        error_code,
        message
    };
}

impl AuthenticationApiException {

    pub fn status(&self) -> u16 {
        return self.status;
    }

    pub fn error_code(&self) -> ErrorCodes::ErrorCodes {
        return self.error_code.clone();
    }

    pub fn message(&self) -> String {
        return self.message.clone();
    }

}