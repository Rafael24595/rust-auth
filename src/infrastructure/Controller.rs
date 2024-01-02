use axum::{
    extract::Path,
    routing::get,
    http::StatusCode,
    Router,
};

use crate::domain::Auth;
use crate::infrastructure::Service;

pub fn route(router: Router) -> Router {
    return router
        .route("/:service/status", get(status))
        .route("/:service/key", get(key))
}

async fn status(Path(service): Path<String>) -> (StatusCode, String) {
    let o_service = Auth::find_service(service.as_str());
    if o_service.is_some() {
        let status = Service::status(service).await;
        if status.is_ok() {
            return (StatusCode::OK, "Service up.".to_string());    
        }
        
        if status.is_err() {
            let error = status.err().unwrap();
            return (StatusCode::from_u16(error.0).unwrap_or_default(), error.1);    
        }

    }
    return (StatusCode::NOT_FOUND, "Not found".to_string());
}

async fn key(Path(service): Path<String>) -> (StatusCode, String) {
    let o_service = Auth::find_service(service.as_str());
    if o_service.is_some() {
        let key = Service::key(service).await;
        if key.is_ok() {
            return (StatusCode::OK, key.unwrap());    
        }
        
        if key.is_err() {
            let error = key.err().unwrap();
            return (StatusCode::from_u16(error.0).unwrap_or_default(), error.1);    
        }

    }
    return (StatusCode::NOT_FOUND, "Not found".to_string());
}