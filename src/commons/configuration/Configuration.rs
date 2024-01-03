use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::domain::PassToken::PassToken;

lazy_static! {
    static ref INSTANCE: Mutex<Option<Configuration>> = Mutex::new(None);
}

#[derive(Clone)]
pub struct Configuration {
    pass_tokens: Vec<PassToken>,
    pub pubkey_name: String,
    pub prikey_name: String
}

pub(crate) fn new(pubkey_name: String, prikey_name: String) -> Configuration {
    Configuration {
        pass_tokens: Vec::new(),
        pubkey_name: pubkey_name,
        prikey_name: prikey_name
    }
}

pub(crate) fn initialize(conf: Configuration) -> Configuration {
    let mut instance = INSTANCE.lock().expect("Could not lock mutex");
    if instance.is_none() {
        *instance = Some(conf);
    } else {
        //TODO: Log.
    }
    return instance.as_ref().unwrap().clone();
}

pub(crate) fn instance() -> Configuration {
    let mut instance = INSTANCE.lock().expect("Could not lock mutex");
    if instance.is_none() {
        //TODO: Log.
    } 

    return instance.as_ref().unwrap().clone();
}

impl Configuration {
    
    pub fn push_token(&mut self, token: PassToken) -> PassToken {
        self.pass_tokens.push(token.clone());
        return token;
    }

}