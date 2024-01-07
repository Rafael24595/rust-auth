#[derive(Clone)]
pub struct PassToken {
    uuid: String,
    owner: String,
    status: PassTokenState,
    message: String
}

pub(crate) fn new(uuid: String, owner: String) -> PassToken {
    PassToken {
        uuid: uuid,
        owner: owner,
        status: PassTokenState::ACTIVE,
        message: String::new()
    }
}

impl PassToken {
    
    pub fn uuid(&self) -> String {
        return self.uuid.clone();
    }

    pub fn status(&self) -> PassTokenState {
        return self.status.clone();
    }

    pub fn is_active(&self) -> bool {
        return self.status == PassTokenState::ACTIVE;
    }

    pub fn owner(&self) -> String {
        return self.owner.clone();
    }

    pub fn exposed(&mut self) {
        self.status = PassTokenState::EXPOSED;
    }

}

#[derive(Clone, PartialEq)]
pub enum PassTokenState {
    ACTIVE,
    EXPIRED,
    EXPOSED,
    NOTFOUND
}

impl PassTokenState {
    pub fn to_string(&self) -> &'static str {
        match self {
            PassTokenState::ACTIVE => "ACTIVE",
            PassTokenState::EXPIRED => "EXPIRED",
            PassTokenState::EXPOSED => "EXPOSED",
            PassTokenState::NOTFOUND => "NOT_FOUND",
        }
    }
}
