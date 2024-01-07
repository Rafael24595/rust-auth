use dotenv::dotenv;
use uuid::Uuid;

use crate::commons::crypto::CryptoConfiguration;
use crate::domain::Services;

use crate::commons::configuration::Configuration;

use crate::domain::PassToken;

pub(crate) fn initialize() {
    dotenv().ok();

    initialize_configuration();
    initialize_services();
}

fn initialize_configuration() -> Configuration::Configuration {
    let pubkey_name = std::env::var("KEY_PUBKEY_NAME");
    let prikey_name = std::env::var("KEY_PRIKEY_NAME");
    let module = std::env::var("KEY_TYPE");
    let format = std::env::var("KEY_FORMAT");
    let pass_phrase = std::env::var("KEY_PASSPHRASE");

    if pubkey_name.is_err() || prikey_name.is_err() {
        //TODO: Exception.
    }

    //TODO: Valide keys.

    let crypto = CryptoConfiguration::new(
        pubkey_name.unwrap(),
        prikey_name.unwrap(),
        module.unwrap(),
        format.unwrap(),
        pass_phrase.unwrap_or_default()
    );

    let conf = Configuration::new(crypto);
    
    Configuration::initialize(conf.clone());
    
    let uuid = Uuid::new_v4().to_string();
    let token = PassToken::new(uuid.clone());
    Configuration::push_token(token);

    println!("{}", uuid);

    return Configuration::instance();
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