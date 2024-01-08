use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct DtoPubKeyResponse {
    pub key: String,
    pub module: String,
    pub format: String,
    pub passphrase: String
}

pub(crate) fn new(key: String, module: String, format: String, passphrase: String) -> DtoPubKeyResponse {
    return DtoPubKeyResponse {
        key,
        module,
        format,
        passphrase
    };
}