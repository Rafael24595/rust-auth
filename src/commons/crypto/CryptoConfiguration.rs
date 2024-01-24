use crate::commons::exception::AuthenticationApiException;
use crate::infrastructure::DtoPubKeyResponse;

use super::modules::asymmetric::AsymmetricKeys;
use super::modules::symmetric::{SymetricKey, SymetricKeys};

#[derive(Clone)]
pub struct CryptoConfiguration {
    asymmetric_key_pair: AsymmetricKeys::AsymmetricKeys,
    symetric_keys: SymetricKeys::SymetricKeys
}

pub(crate) fn new(asymmetric_key_pair: AsymmetricKeys::AsymmetricKeys, symetric_keys: SymetricKeys::SymetricKeys) -> CryptoConfiguration {
    CryptoConfiguration {
        asymmetric_key_pair: asymmetric_key_pair,
        symetric_keys: symetric_keys
    }
}

impl CryptoConfiguration {

    pub fn asymmetric_key_pair(&self) -> AsymmetricKeys::AsymmetricKeys {
        return self.asymmetric_key_pair.clone();
    }

    pub fn symmetric_key(&mut self) -> Result<SymetricKey::SymetricKey, AuthenticationApiException::AuthenticationApiException> {
        return self.symetric_keys.find();
    }

    pub fn read_public(&self) -> DtoPubKeyResponse::DtoPubKeyResponse {
        return self.asymmetric_key_pair.public_key();
    }

}