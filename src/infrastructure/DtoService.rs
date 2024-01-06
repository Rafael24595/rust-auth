use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DtoService {
    pub pass_key: String,
    pub end_point_status: String,
    pub end_point_key: String,
}

pub(crate) fn new(pass_key: String, end_point_status: String, end_point_key: String) -> DtoService {
    DtoService {
        pass_key,
        end_point_status,
        end_point_key,
    }
}