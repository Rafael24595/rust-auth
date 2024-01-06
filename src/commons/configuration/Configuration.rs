use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::domain::PassToken::PassToken;
use crate::commons::crypto::CryptoConfiguration::CryptoConfiguration;

lazy_static! {
    static ref INSTANCE: Mutex<Option<Configuration>> = Mutex::new(None);
}

#[derive(Clone)]
pub struct Configuration {
    pass_tokens: Vec<PassToken>,
    pub crypto: CryptoConfiguration,
}

pub(crate) fn new(crypto: CryptoConfiguration) -> Configuration {
    Configuration {
        pass_tokens: Vec::new(),
        crypto: crypto
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
    let instance = INSTANCE.lock().expect("Could not lock mutex");
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