use axum::{
    extract::{Json, Host, Path, Request},
    response::Response,
    routing::{get, post},
    http::StatusCode,
    Router, body::{Body, to_bytes},
};

use crate::infrastructure::{
    Service, DtoService, DtoPubKeyResponse,
    entity::{CryptoRequest, HeaderParameter, QueryParameter}
};

pub fn route(router: Router) -> Router {
    return router
        .route("/nodekey", get(nodekey))
        .route("/:service/subscribe", post(subscribe))
        .route("/:service/status", get(status))
        .route("/:service/key", get(key))

        .route("/:service/resolve/*path", 
            get(resolve)
            .head(resolve)
            .post(resolve)
            .put(resolve)
            .delete(resolve)
            .options(resolve)
            .trace(resolve)
            .patch(resolve)
        )
}

async fn nodekey() -> Result<(StatusCode, Json<DtoPubKeyResponse::DtoPubKeyResponse>), (StatusCode, String)> {
    let key = Service::nodekey().await;
    
    if key.is_ok() {
        return Ok((StatusCode::ACCEPTED, Json(key.unwrap())));    
    }
    
    if key.is_err() {
        let error = key.err().unwrap();
        return Err((StatusCode::from_u16(error.status()).unwrap_or_default(), error.message()));
    }

    return Err((StatusCode::NOT_FOUND, "Not found".to_string()));
}

async fn subscribe(Host(hostname): Host, Path(service): Path<String>, Json(dto): Json<DtoService::DtoService>) -> Result<(StatusCode, String), (StatusCode, String)> {
    let status = Service::subscribe(service, hostname, dto).await;

    if status.is_ok() {
        return Ok((StatusCode::ACCEPTED, String::from("Service subscribed successfully.")));    
    }
    
    if status.is_err() {
        let error = status.err().unwrap();
        return Err((StatusCode::from_u16(error.status()).unwrap_or_default(), error.message()));   
    }
    return Err((StatusCode::NOT_FOUND, "Not found".to_string()));
}

async fn status(Path(service): Path<String>) -> Result<(StatusCode, String), (StatusCode, String)> {
    let status = Service::status(service).await;

    if status.is_ok() {
        return Ok((StatusCode::ACCEPTED, String::from("Service up.")));    
    }
    
    if status.is_err() {
        let error = status.err().unwrap();
        return Err((StatusCode::from_u16(error.status()).unwrap_or_default(), error.message()));
    }

    return Err((StatusCode::NOT_FOUND, "Not found".to_string()));
}

async fn key(Path(service): Path<String>) -> Result<(StatusCode, Json<DtoPubKeyResponse::DtoPubKeyResponse>), (StatusCode, String)> {
    let key = Service::key(service).await;

    if key.is_ok() {
        return Ok((StatusCode::ACCEPTED, Json(key.unwrap())));    
    }
    
    if key.is_err() {
        let error = key.err().unwrap();
        return Err((StatusCode::from_u16(error.status()).unwrap_or_default(), error.message()));    
    }

    return Err((StatusCode::NOT_FOUND, String::from("Not found")));
}

async fn resolve(Path((service, path)): Path<(String, String)>, request: Request) ->  Response<Body> {
    let method = request.method().to_string();
    let headers = request.headers().clone();
    let uri = request.uri().clone();
    let query = uri.query().unwrap_or_default();
    let mut body = Vec::new();
    let b_body = to_bytes(request.into_body(), usize::MAX).await;
    if b_body.is_ok() {
        body = b_body.unwrap().to_vec();
        //body = String::from_utf8_lossy(&b_body.unwrap()).to_string();
    }

    let mut crypto_request = CryptoRequest::new();
    crypto_request.setMethod(method);
    crypto_request.setService(service);
    crypto_request.setPath(path);
    crypto_request.setQuery(query);
    crypto_request.setBody(body);

    for (o_name, b_value) in headers {
        let name = o_name.unwrap().to_string();
        let value = b_value.to_str().unwrap().to_string();
        crypto_request.addHeaderParameterTuple(name, value);
    }
    
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from("Hello world!"))
        .unwrap();
    return response;
}