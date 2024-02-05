use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct DtoPubKeyResponse {
    pub key: String,
    pub module: String,
    pub format: String,
    pub pass_phrase: String
}

pub(crate) fn new(key: String, module: String, format: String, pass_phrase: String) -> DtoPubKeyResponse {
    return DtoPubKeyResponse {
        key,
        module,
        format,
        pass_phrase
    };
}