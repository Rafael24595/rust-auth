use crate::commons::exception::AuthenticationAppException;

#[derive(Clone)]
pub(crate) enum AesBytes {
    B128,
    B192,
    B256
}

pub(crate) fn from_usize(bytes: usize) -> Result<AesBytes, AuthenticationAppException::AuthenticationAppException> {
    let result = match bytes {
        128 => AesBytes::B128,
        192 => AesBytes::B192,
        256 => AesBytes::B256,
        _ => {
            return Err(AuthenticationAppException::new(String::from("AES Bytes value must be 128, 192 or 256")));
        }
    };
    return Ok(result);
}

impl AesBytes {
    pub fn as_usize(&self) -> usize {
        match self {
            Self::B128 => 128,
            Self::B192 => 192,
            Self::B256 => 256,
        }
    }
}