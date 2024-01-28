use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DtoSymetricKey {
    pub module: String,
    pub key: String,
    pub format: String,
    pub expires: u128
}

pub(crate) fn new(module: String, key: String, format: String, expires: u128) -> DtoSymetricKey {
    return DtoSymetricKey {
        module: module,
        key: key,
        format: format,
        expires: expires,
    };
}