use lazy_static::lazy_static;
use uuid::Uuid;
use std::ops::IndexMut;
use std::sync::Mutex;

use crate::domain::PassToken;
use crate::commons::crypto::CryptoConfiguration::CryptoConfiguration;

pub const SELF_OWNER: &str = "ADMIN_CERBERUS";
pub const COOKIE_NAME: &str = "pass-token";
pub const HEADER_INTEGRITY_NAME: &str = "crypto-integrity";

lazy_static! {
    static ref INSTANCE: Mutex<Option<Configuration>> = Mutex::new(None);
}

#[derive(Clone)]
pub struct Configuration {
    self_owner: String,
    pass_tokens: Vec<PassToken::PassToken>,
    pub crypto: CryptoConfiguration,
}

pub(crate) fn new(self_owner: Option<String>, crypto: CryptoConfiguration) -> Configuration {
    let mut owner = SELF_OWNER.to_string();
    if self_owner.is_some() && !self_owner.as_ref().unwrap().trim().is_empty() {
        owner = self_owner.unwrap();
    }
    Configuration {
        self_owner: owner,
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

    let token = PassToken::new(uuid.clone(), instance().self_owner);
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
    let conf = binding.as_mut().unwrap();
    let o_index = conf.pass_tokens.iter().position(|t| t.uuid() == uuid);
    if let Some(index) = o_index {
        let pass = conf.pass_tokens.index_mut(index);
        pass.exposed();
        if pass.owner() == instance().self_owner {
            create_service_token();
        }
        return Some(pass.clone());
    }
    return None;
}

pub(crate) fn includes_active_token(message: String) -> Result<String, ()> {
    let instance = instance();
    for token in instance.pass_tokens {
        if token.is_active() && token_finder(message.clone(), token.uuid()) {
            return Ok(token.uuid());
        }
    }
    return Err(());
}

fn token_finder(message: String, token: String) -> bool {
    let normal_coincidence = find_key(token.clone(), message.clone());
    if normal_coincidence.eq_ignore_ascii_case(&token) {
        return true;
    }

    let reverse_token = token.chars().rev().collect::<String>();
    let reverse_coincidence = find_key(reverse_token.clone(), message.clone());
    if reverse_coincidence.eq_ignore_ascii_case(&reverse_token) {
        return true;
    }

    let fragments_normal = find_key_fragments(token.clone(), message.clone());
    let fragments_reversed = find_key_fragments(reverse_token.clone(), message.clone());
    let mut fragments: Vec<String> = fragments_reversed.iter().map(|f| f.chars().rev().collect::<String>()).collect();
    fragments.append(&mut fragments_normal.clone());

    for fragment in fragments {
        let percentage = fragment.len() as f64 / token.len() as f64;
        if percentage > 0.85 && token.contains(&fragment) {
            return true;       
        }
    }

    return false;
}

fn find_key_fragments(key: String, input: String) -> Vec<String> {
    let mut fragments = Vec::<String>::new();
    for (index, _) in key.chars().enumerate() {
        let substring = &key[index..];
        let result = find_key(substring.to_string(), input.clone());
        if !result.is_empty() {
            fragments.push(result);
        }
    }
    return fragments;
}

fn find_key(key: String, input: String) -> String {
    let mut last_coincidence = String::new();
    for char in input.to_lowercase().chars() {
        let coincidence = last_coincidence.clone() + &char.to_string();
        if key.starts_with(&coincidence) {
            last_coincidence = coincidence;
        }
    }

    return last_coincidence;
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