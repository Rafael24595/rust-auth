use lazy_static::lazy_static;
use uuid::Uuid;
use std::ops::IndexMut;
use std::sync::Mutex;

use crate::domain::PassToken;
use crate::commons::crypto::CryptoConfiguration::CryptoConfiguration;

pub const SELF_OWNER: &str = "ADMIN_CERBERUS";

lazy_static! {
    static ref INSTANCE: Mutex<Option<Configuration>> = Mutex::new(None);
}

#[derive(Clone)]
pub struct Configuration {
    pass_tokens: Vec<PassToken::PassToken>,
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

pub(crate) fn create_service_token() -> PassToken::PassToken {
    let uuid = Uuid::new_v4().to_string();
    if find_token(uuid.clone()) {
       return create_service_token();
    }

    let token = PassToken::new(uuid.clone(), SELF_OWNER.to_string());
    let _ = push_token(token.clone());
    return token;
}

pub(crate) fn push_token(token: PassToken::PassToken) -> Result<PassToken::PassToken, String> {
    let mut binding = INSTANCE.lock().expect("Could not lock mutex");
    let instance = binding.as_mut();
    instance.unwrap().pass_tokens.push(token.clone());
    return Ok(token);
}

pub(crate) fn deprecate_token(uuid: String) -> Option<PassToken::PassToken> {
    let mut binding = INSTANCE.lock().expect("Could not lock mutex");
    let instance = binding.as_mut().unwrap();
    let o_index = instance.pass_tokens.iter().position(|t| t.uuid() == uuid);
    if let Some(index) = o_index {
        let pass = instance.pass_tokens.index_mut(index);
        pass.exposed();
        if pass.owner() == SELF_OWNER {
            create_service_token();
        }
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

pub(crate) fn find_token(uuid: String) -> bool {
    let instance = instance();
    for token in instance.pass_tokens {
        if uuid == token.uuid() {
            return true
        }
    }
    return false;
}

pub(crate) fn find_active_token(uuid: String) -> Result<(), PassToken::PassTokenState> {
    let instance = instance();
    for token in instance.pass_tokens {
        if uuid == token.uuid() {
            if token.is_active() {
                return Ok(());
            }
            return Err(token.status());
        }
    }
    return Err(PassToken::PassTokenState::NOTFOUND);
}