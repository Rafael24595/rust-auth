#[derive(Debug, Clone)]
pub enum ErrorCodes {
    CLIUA001,
    CLIFB001,
}

impl ErrorCodes {

    pub fn code(&self) -> &str {
        return self.data().code;
    }
    
    pub fn data(&self) -> ErrorCode {
        match *self {
            ErrorCodes::CLIUA001 => ErrorCode{code: "CLIUA001", description: ""},
            ErrorCodes::CLIFB001 => ErrorCode{code: "CLIFB001", description: ""},
        }
    }

}

#[derive(Debug, Clone)]
pub struct ErrorCode {
    pub code: &'static str,
    pub description: &'static str,
}