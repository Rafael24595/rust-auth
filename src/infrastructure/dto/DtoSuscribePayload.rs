use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DtoSuscribePayload {
    pub payload: String
}

pub(crate) fn new(payload: String) -> DtoSuscribePayload {
    DtoSuscribePayload {
        payload
    }
}