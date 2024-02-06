use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::infrastructure::dto::DtoExceptionData;

#[derive(Debug, Clone, EnumIter)]
pub enum ErrorCodes {
    SYSIN001, SYSIN002, SYSIN003,

    CLIDT001, CLIDT002, CLIDT003,

    CLIUA001, CLIUA002, CLIUA003,
    CLIUA004, CLIUA005, CLIUA006,
    CLIUA007, CLIUA008,

    CLIFB001, CLIFB002, CLIFB003,
    CLIFB004, CLIFB005, CLIFB006,
    CLIFB007, CLIFB008,
}

pub(crate) fn from_slice(code: String) -> Option<ErrorCodes> {
    for error in ErrorCodes::iter() {
        if code == error.code() {
            return Some(error);
        }
    }
    return None;
}

impl ErrorCodes {

    pub fn code(&self) -> &str {
        return self.data().code;
    }
    
    pub fn data(&self) -> ErrorCode {
        match *self {
            ErrorCodes::SYSIN001 => ErrorCode{code: "SYSIN001", description: "Internal server error."},
            ErrorCodes::SYSIN002 => ErrorCode{code: "SYSIN002", description: "Service input cannot be processed by server."},
            ErrorCodes::SYSIN003 => ErrorCode{code: "SYSIN003", description: "Future implementation."},

            ErrorCodes::CLIDT001 => ErrorCode{code: "CLIDT001", description: "Service bad status."},
            ErrorCodes::CLIDT002 => ErrorCode{code: "CLIDT002", description: "Service public key data cannot be processed."},
            ErrorCodes::CLIDT003 => ErrorCode{code: "CLIDT003", description: "Service bad response."},

            ErrorCodes::CLIUA001 => ErrorCode{code: "CLIUA001", description: "Service is not registered."},
            ErrorCodes::CLIUA002 => ErrorCode{code: "CLIUA002", description: "Service token not found."},
            ErrorCodes::CLIUA003 => ErrorCode{code: "CLIUA003", description: "Service token bad format."},
            ErrorCodes::CLIUA004 => ErrorCode{code: "CLIUA004", description: "Symmetric key format unsupported."},
            ErrorCodes::CLIUA005 => ErrorCode{code: "CLIUA005", description: "Service is already registered."},
            ErrorCodes::CLIUA006 => ErrorCode{code: "CLIUA006", description: "Unautorized pass token."},
            ErrorCodes::CLIUA007 => ErrorCode{code: "CLIUA007", description: "Pass token exposed."},
            ErrorCodes::CLIUA008 => ErrorCode{code: "CLIUA008", description: "Non valid subscribe payload format."},
            
            ErrorCodes::CLIFB001 => ErrorCode{code: "CLIFB001", description: "Symmetric key is not defined."},
            ErrorCodes::CLIFB002 => ErrorCode{code: "CLIFB002", description: "Symmetric key is not active."},
            ErrorCodes::CLIFB003 => ErrorCode{code: "CLIFB003", description: "Service token expired."},
            ErrorCodes::CLIFB004 => ErrorCode{code: "CLIFB004", description: "Token integrity compromised."},
            ErrorCodes::CLIFB005 => ErrorCode{code: "CLIFB005", description: "Incorrect symmetric key data."},
            ErrorCodes::CLIFB006 => ErrorCode{code: "CLIFB006", description: "Incorrect encrypted message format."},
            ErrorCodes::CLIFB007 => ErrorCode{code: "CLIFB007", description: "Message cannot be decrypted."},
            ErrorCodes::CLIFB008 => ErrorCode{code: "CLIFB008", description: "Message cannot be encrypted."},
        }
    }

    pub fn as_dto(&self) -> DtoExceptionData::DtoExceptionData {
        let data = self.data();
        return DtoExceptionData::new(data.code.to_string(), data.description.to_string());
    }

}

#[derive(Debug, Clone)]
pub struct ErrorCode {
    pub code: &'static str,
    pub description: &'static str,
}