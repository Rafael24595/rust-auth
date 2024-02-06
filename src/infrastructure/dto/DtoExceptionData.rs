use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct DtoExceptionData {
    pub code: String,
    pub description: String,
}

pub(crate) fn new(code: String, description: String) -> DtoExceptionData {
    return DtoExceptionData {
        code,
        description
    };
}