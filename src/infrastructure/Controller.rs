use axum::{
    extract::{Json, Host},
    extract::Path,
    routing::{get, post},
    http::StatusCode,
    Router,
};

use crate::infrastructure::{Service, DtoService};

pub fn route(router: Router) -> Router {
    return router
        .route("/nodekey", get(nodekey))
        .route("/:service/subscribe", post(subscribe))
        .route("/:service/status", get(status))
        .route("/:service/key", get(key))
}

async fn nodekey() -> (StatusCode, String) {
    let key = Service::nodekey().await;
    
    if key.is_ok() {
        return (StatusCode::OK, key.unwrap());    
    }
    
    if key.is_err() {
        let error = key.err().unwrap();
        return (StatusCode::INTERNAL_SERVER_ERROR, error);    
    }

    return (StatusCode::NOT_FOUND, "Not found".to_string());
}

async fn subscribe(Host(hostname): Host, Path(service): Path<String>, Json(dto): Json<DtoService::DtoService>) -> (StatusCode, String) {
    let status = Service::subscribe(service, hostname, dto).await;

    if status.is_ok() {
        return (StatusCode::OK, "Service up.".to_string());    
    }
    
    if status.is_err() {
        let error = status.err().unwrap();
        return (StatusCode::INTERNAL_SERVER_ERROR, error);    
    }
    return (StatusCode::NOT_FOUND, "Not found".to_string());
}

async fn status(Path(service): Path<String>) -> (StatusCode, String) {
    let status = Service::status(service).await;

    if status.is_ok() {
        return (StatusCode::OK, "Service up.".to_string());    
    }
    
    if status.is_err() {
        let error = status.err().unwrap();
        return (StatusCode::from_u16(error.0).unwrap_or_default(), error.1);    
    }

    return (StatusCode::NOT_FOUND, "Not found".to_string());
}

async fn key(Path(service): Path<String>) -> (StatusCode, String) {
    let key = Service::key(service).await;

    if key.is_ok() {
        return (StatusCode::OK, key.unwrap());    
    }
    
    if key.is_err() {
        let error = key.err().unwrap();
        return (StatusCode::from_u16(error.0).unwrap_or_default(), error.1);    
    }

    return (StatusCode::NOT_FOUND, "Not found".to_string());
}