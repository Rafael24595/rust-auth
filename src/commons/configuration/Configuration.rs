use lazy_static::lazy_static;
use std::ops::IndexMut;
use std::sync::Mutex;

use crate::domain::PassToken::{PassToken, PassTokenState};
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

pub(crate) fn push_token(token: PassToken) -> PassToken {
    let mut binding = INSTANCE.lock().expect("Could not lock mutex");
    let instance = binding.as_mut();
    instance.unwrap().pass_tokens.push(token.clone());
    return token;
}

pub(crate) fn deprecate_token(uuid: String) -> Option<PassToken> {
    let mut binding = INSTANCE.lock().expect("Could not lock mutex");
    let instance = binding.as_mut().unwrap();
    let o_index = instance.pass_tokens.iter().position(|t| t.uuid() == uuid);
    if let Some(index) = o_index {
        let pass = instance.pass_tokens.index_mut(index);
        pass.exposed();
        return Some(pass.clone());
    }
    return None;
}

pub(crate) fn includes_active_token(message: String) -> Result<String, ()> {
    let instance = instance();
    for token in instance.pass_tokens {
        if token.is_active() && message.contains(&token.uuid()) {
            return Ok(token.uuid());
        }
    }
    return Err(());
}

pub(crate) fn find_active_token(uuid: String) -> Result<(), PassTokenState> {
    let instance = instance();
    for token in instance.pass_tokens {
        if uuid == token.uuid() {
            if token.is_active() {
                return Ok(());
            }
            return Err(token.status());
        }
    }
    return Err(PassTokenState::NOTFOUND);
}