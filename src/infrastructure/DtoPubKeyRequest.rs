use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct DtoPubKeyRequest {
    pub key: String,
    pub module: String,
    pub format: String,
    pub passphrase: String,
    pub expires: u128
}

pub(crate) fn new(key: String, module: String, format: String, passphrase: String, expires: u128) -> DtoPubKeyRequest {
    return DtoPubKeyRequest {
        key,
        module,
        format,
        passphrase,
        expires
    };
}