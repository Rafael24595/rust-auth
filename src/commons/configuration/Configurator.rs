use dotenv::dotenv;

use crate::commons::crypto::modules::asymmetric::{AsymmetricKeys, AsymmetricPrivate, AsymmetricPublic, Utils};
use crate::commons::crypto::modules::symmetric::SymetricKeys;
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
    
    initialize_services(result.unwrap());
    initialize_pass_tokens();

    return Ok(());
}

fn initialize_configuration() -> Result<PassToken::PassToken, AuthenticationAppException::AuthenticationAppException> {
    let self_owner = std::env::var("SELF_OWNER");

    let asymmetric = build_asymmetric_data()?;
    let symmetric = build_symmetric_data()?;
    
    let crypto = CryptoConfiguration::new(
        asymmetric,
        symmetric,
    );


    let conf = Configuration::new(self_owner.ok(), crypto);
    
    Configuration::initialize(conf.clone());

    let token = create_service_token();

    println!("Service token created: {}", token.uuid());

    return Ok(token);
}

fn build_asymmetric_data() -> Result<AsymmetricKeys::AsymmetricKeys, AuthenticationAppException::AuthenticationAppException> {
    let pubkey_name = std::env::var("KEY_PUBKEY_NAME");
    let prikey_name = std::env::var("KEY_PRIKEY_NAME");
    let module = std::env::var("KEY_TYPE").unwrap_or_default();
    let format = std::env::var("KEY_FORMAT").unwrap_or_default();
    let pass_phrase = std::env::var("KEY_PASSPHRASE").unwrap_or_default();
    let s_expires_range = std::env::var("EXPIRES_RANGE");

    if pubkey_name.is_err() || prikey_name.is_err() {
        return Err(AuthenticationAppException::new(String::from("Incorrect number of arguments.")));
    }

    let r_expires_range = s_expires_range.unwrap_or(String::from("900000"));

    let priv_key = Utils::read_key(prikey_name.unwrap());
    if priv_key.is_err() {
        return Err(AuthenticationAppException::new(priv_key.err().unwrap().to_string()));
    }
    let prikey = AsymmetricPrivate::new(
        priv_key.unwrap(),
        module.clone(),
        format.clone(),
        pass_phrase.clone(),
        r_expires_range.parse().unwrap()
    );

    let publ_key = Utils::read_key(pubkey_name.unwrap());
    if publ_key.is_err() {
        return Err(AuthenticationAppException::new(publ_key.err().unwrap().to_string()));
    }
    let pubkey = AsymmetricPublic::new(
        publ_key.unwrap(),
        module.clone(),
        format.clone(),
        pass_phrase.clone(),
        r_expires_range.parse().unwrap(),
        u128::MAX
    );

    let asymmetric = AsymmetricKeys::new(
        pubkey,
        prikey
    );

    let _ = asymmetric.evalue()?;

    return Ok(asymmetric);
}

fn build_symmetric_data() -> Result<SymetricKeys::SymetricKeys, AuthenticationAppException::AuthenticationAppException> {
    let module = std::env::var("SYMM_KEY_TYPE");
    let format = std::env::var("SYMM_KEY_FORMAT");
    let s_expires_range = std::env::var("SYMM_EXPIRES");

    let r_expires_range = s_expires_range
        .unwrap_or(String::from("1800000"))
        .parse::<u128>()
        .unwrap_or(1800000);

    let mut symmetric = SymetricKeys::new(
        module.unwrap(),
        format.unwrap(),
        r_expires_range,
    )?;

    let key = symmetric.generate_new()?;

    let _ = key.evalue()?;

    return Ok(symmetric);
}

pub(crate) fn initialize_services(token: PassToken::PassToken) {
    let services = find_services();
    for service in services {
        let uri = std::env::var(service.to_uppercase() + "_URI");
        let end_point_status = std::env::var(service.to_uppercase() + "_STATUS");
        let end_point_key = std::env::var(service.to_uppercase() + "_KEY");
        if uri.is_ok() && end_point_status.is_ok() && end_point_key.is_ok() {
            Services::insert(service, uri.unwrap(), token.uuid(), end_point_status.unwrap(), end_point_key.unwrap());
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