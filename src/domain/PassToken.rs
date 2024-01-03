#[derive(Clone)]
pub struct PassToken {
    uuid: String,
    state: PassTokenState,
    message: String
}

pub(crate) fn new(uuid: String) -> PassToken {
    PassToken {
        uuid: uuid,
        state: PassTokenState::ACTIVE,
        message: String::new()
    }
}

#[derive(Clone)]
pub enum PassTokenState {
    ACTIVE,
    EXPIRED,
    EXPOSED
}