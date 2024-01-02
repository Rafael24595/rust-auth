use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct DtoKey {
    pub public: String,
    pub expires: u128
}

fn new(public: String, expires: u128) -> DtoKey {
    DtoKey {
        public,
        expires
    }
}