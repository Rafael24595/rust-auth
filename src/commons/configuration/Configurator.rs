use dotenv::dotenv;

use crate::commons::crypto::CryptoConfiguration;
use crate::commons::exception::AuthenticationAppException;
use crate::domain::{Services, PassToken};

use crate::commons::configuration::Configuration::{self, create_service_token};

pub(crate) fn initialize() -> Result<(), AuthenticationAppException::AuthenticationAppException> {
    dotenv().ok();

    let result = initialize_configuration();
    if result.is_err() {
        return Err(result.err().unwrap());
    }
    
    initialize_services();
    initialize_pass_tokens();

    return Ok(());
}

fn initialize_configuration() -> Result<Configuration::Configuration, AuthenticationAppException::AuthenticationAppException> {
    let self_owner = std::env::var("SELF_OWNER");

    let pubkey_name = std::env::var("KEY_PUBKEY_NAME");
    let prikey_name = std::env::var("KEY_PRIKEY_NAME");
    let module = std::env::var("KEY_TYPE");
    let format = std::env::var("KEY_FORMAT");
    let pass_phrase = std::env::var("KEY_PASSPHRASE");
    let s_expires_range = std::env::var("EXPIRES_RANGE");

    if pubkey_name.is_err() || prikey_name.is_err() {
        return Err(AuthenticationAppException::new(String::from("Incorrect number of arguments.")));
    }

    let r_expires_range = s_expires_range.unwrap_or(String::from("900000"));

    let crypto = CryptoConfiguration::new(
        pubkey_name.unwrap(),
        prikey_name.unwrap(),
        module.unwrap(),
        format.unwrap(),
        pass_phrase.unwrap_or_default(),
        r_expires_range.parse().unwrap()
    );

    let result = crypto.evalue();
    if result.is_err() {
        return Err(result.err().unwrap());
    }

    let conf = Configuration::new(self_owner.ok(), crypto);
    
    Configuration::initialize(conf.clone());

    let token = create_service_token();

    println!("Service token created: {}", token.uuid());

    return Ok(Configuration::instance());
}

pub(crate) fn initialize_services() {
    let services = find_services();
    for service in services {
        let uri = std::env::var(service.to_uppercase() + "_URI");
        let end_point_status = std::env::var(service.to_uppercase() + "_STATUS");
        let end_point_key = std::env::var(service.to_uppercase() + "_KEY");
        if uri.is_ok() && end_point_status.is_ok() && end_point_key.is_ok() {
            Services::insert(service, uri.unwrap(), end_point_status.unwrap(), end_point_key.unwrap());
        } else {
            //TODO: Log.
        }
    }
}

fn find_services() -> Vec<String> {
    let services_chain = std::env::var("SERVICE_CODES");
    if services_chain.is_ok() {
        return services_chain.unwrap()
            .split("&")
            .map(|s| s.to_string())
            .collect();
    }
    return Vec::new();
}

pub(crate) fn initialize_pass_tokens() {
    let owners = find_pass_tokens();
    for owner in owners {
        let uuid = std::env::var(owner.to_uppercase() + "_UUID");
        if uuid.is_ok() {
            let token = PassToken::new(uuid.unwrap().clone(), owner);
            let _ = Configuration::push_token(token.clone());
        } else {
            //TODO: Log.
        }
    }
}

fn find_pass_tokens() -> Vec<String> {
    let ownsers_chain = std::env::var("PASS_TOKEN_OWNERS");
    if ownsers_chain.is_ok() {
        return ownsers_chain.unwrap()
            .split("&")
            .map(|s| s.to_string())
            .collect();
    }
    return Vec::new();
}