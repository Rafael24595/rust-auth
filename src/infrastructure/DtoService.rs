use serde::Deserialize;

use super::DtoSymetricKey;

#[derive(Clone, Deserialize)]
pub struct DtoService {
    pub service: String,
    pub pass_key: String,
    pub symetric_key: DtoSymetricKey::DtoSymetricKey,
    pub host: String,
    pub end_point_status: String,
    pub end_point_key: String,
}

pub(crate) fn new(service: String, pass_key: String, symetric_key: DtoSymetricKey::DtoSymetricKey, host: String, end_point_status: String, end_point_key: String) -> DtoService {
    DtoService {
        service,
        pass_key,
        symetric_key,
        host,
        end_point_status,
        end_point_key,
    }
}