use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct DtoPubKeyRequest {
    pub key: String,
    pub module: String,
    pub format: String,
    pub pass_phrase: String,
    pub expires: u128
}

pub(crate) fn new(key: String, module: String, format: String, pass_phrase: String, expires: u128) -> DtoPubKeyRequest {
    return DtoPubKeyRequest {
        key,
        module,
        format,
        pass_phrase,
        expires
    };
}