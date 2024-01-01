use axum::{
    extract::Path,
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

use serde::{Deserialize, Serialize};

use crate::domain::Auth;

pub fn route(router: Router) -> Router {
    return router
        .route("/:service/status", get(status))
}

async fn status(Path(service): Path<String>) -> (StatusCode, String) {
    let o_service = Auth::find_service(service.as_str());
    if o_service.is_some() {
        return (StatusCode::OK, o_service.unwrap().code() + " => " + &o_service.unwrap().uri());
    }
    return (StatusCode::NOT_FOUND, "Not found".to_string());
}