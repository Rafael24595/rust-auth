use dotenv::dotenv;

use crate::domain::Auth;

pub(crate) fn initialize() {
    dotenv().ok();
    let services = find_services();
    for service in services {
        let uri = std::env::var(service.to_uppercase() + "_URI");
        let end_point_status = std::env::var(service.to_uppercase() + "_STATUS");
        let end_point_key = std::env::var(service.to_uppercase() + "_KEY");
        if uri.is_ok() && end_point_status.is_ok() && end_point_key.is_ok() {
            Auth::insert_service(service, uri.unwrap(), end_point_status.unwrap(), end_point_key.unwrap());
        }
        //TODO: Log.
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