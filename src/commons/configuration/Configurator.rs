use dotenv::dotenv;
use uuid::Uuid;

use crate::domain::Auth;

use crate::commons::configuration::Configuration;

use crate::domain::PassToken;

pub(crate) fn initialize() {
    dotenv().ok();

    initialize_configuration();
    initialize_services();
}

fn initialize_configuration() -> Configuration::Configuration {
    let pubkey_name = std::env::var("PUBKEY_NAME");
    let prikey_name = std::env::var("PRIKEY_NAME");

    if pubkey_name.is_err() || prikey_name.is_err() {
        //TODO: Exception.
    }

    //TODO: Valide keys.

    let mut conf = Configuration::new(pubkey_name.unwrap(), prikey_name.unwrap());
    let uuid = Uuid::new_v4().to_string();
    let token = PassToken::new(uuid);
    conf.push_token(token);

    Configuration::initialize(conf.clone());

    return conf;
}

pub(crate) fn initialize_services() {
    let services = find_services();
    for service in services {
        let uri = std::env::var(service.to_uppercase() + "_URI");
        let end_point_status = std::env::var(service.to_uppercase() + "_STATUS");
        let end_point_key = std::env::var(service.to_uppercase() + "_KEY");
        if uri.is_ok() && end_point_status.is_ok() && end_point_key.is_ok() {
            Auth::insert_service(service, uri.unwrap(), end_point_status.unwrap(), end_point_key.unwrap());
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