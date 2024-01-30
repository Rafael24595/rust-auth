use axum::{
    extract::{Json, Path, Request},
    routing::{get, post},
    body::{Body, to_bytes}, 
    response::Response, http::StatusCode,
    Router, middleware,
};

use crate::{commons::{configuration::Configuration, crypto::ServiceToken}, domain::Services, infrastructure::{
    entity::CryptoRequest,
    Handler, Service, DtoPubKeyResponse}};

use crate::domain::Service as WebService;

use super::DtoSuscribePayload;

pub fn route(router: Router) -> Router {
    return router    
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
        .route("/:service/status", get(status))
        .route("/:service/key", get(key))
        .route_layer(middleware::from_fn(Handler::auth_handler))

        .route("/nodekey", get(nodekey))
        .route("/subscribe", post(subscribe))
        .route("/renove", post(renove))
        .route_layer(middleware::from_fn(Handler::client_tracer_handler))
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

async fn subscribe(Json(dto): Json<DtoSuscribePayload::DtoSuscribePayload>) -> Result<(StatusCode, String), (StatusCode, String)> {
    let status = Service::subscribe(dto).await;

    if status.is_ok() {
        return Ok((StatusCode::ACCEPTED, status.unwrap()));    
    }
    
    if status.is_err() {
        let error = status.err().unwrap();
        return Err((StatusCode::from_u16(error.status()).unwrap_or_default(), error.message()));   
    }
    return Err((StatusCode::NOT_FOUND, "Not found".to_string()));
}

async fn renove(Json(dto): Json<DtoSuscribePayload::DtoSuscribePayload>) -> Result<(StatusCode, String), (StatusCode, String)> {
    let status = Service::renove(dto).await;

    if status.is_ok() {
        return Ok((StatusCode::ACCEPTED, status.unwrap()));    
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

async fn resolve(Path((service, path)): Path<(String, String)>, request: Request) -> Result<Response<Body>, (StatusCode, String)>  {
    let header = request.headers().get(String::from(Configuration::COOKIE_NAME));
    let web_service = token_service(header)?;

    let method = request.method().to_string();
    let headers = request.headers().clone();
    let uri = request.uri().clone();
    let query = uri.query().unwrap_or_default();
    let mut body = Vec::new();
    let b_body = to_bytes(request.into_body(), usize::MAX).await;
    if b_body.is_ok() {
        if web_service.symetric_key().is_none() {
            return Err((StatusCode::FORBIDDEN, String::from("Symmetric key not found"))); 
        }
        let decrypted = web_service.symetric_key().unwrap().decrypt_message(&b_body.unwrap().to_vec());
        if decrypted.is_err() {
            return Err((StatusCode::FORBIDDEN, decrypted.err().unwrap().message())); 
        }
        body = decrypted.unwrap().as_bytes().to_vec();
    }

    let mut crypto_request = CryptoRequest::new();
    crypto_request.set_method(method);
    crypto_request.set_service(service);
    crypto_request.set_path(path);
    crypto_request.set_query(query);
    crypto_request.set_body(body);

    for (o_name, b_value) in headers {
        let name = o_name.unwrap().to_string();
        let value = b_value.to_str().unwrap().to_string();
        crypto_request.add_header_parameter_tuple(name, value);
    }

    let r_crypto_response = Service::resolve(crypto_request).await;
    if r_crypto_response.is_err() {
        let error = r_crypto_response.err().unwrap();
        return Err((StatusCode::from_u16(error.status()).unwrap_or_default(), error.message()));    
    }
    let crypto_response = r_crypto_response.unwrap();

    let mut response = Response::builder();
    response = response.status(crypto_response.status());

    for header in crypto_response.headers() {
        let key = header.key();
        for value in header.values() {
            response = response.header(key.clone(), value);
        }
    }
    
    let r_response = response.body(Body::from(crypto_response.body()));
    if r_response.is_err() {
        let error = r_response.err().unwrap();
        return Err((StatusCode::INTERNAL_SERVER_ERROR, error.to_string()));    
    }

    return Ok(r_response.unwrap());
}

fn token_service(o_token: Option<&axum::http::HeaderValue>) -> Result<WebService::Service, (StatusCode, String)>  {
    if o_token.is_none() {
        return Err((StatusCode::UNAUTHORIZED, String::from("Token not found")));
    }
    let token = o_token.unwrap().to_str().unwrap().to_string();
    let service_token = ServiceToken::from_string(token);
    if service_token.is_err() {
        return Err((StatusCode::UNAUTHORIZED, service_token.err().unwrap().message()));
    }

    let o_service = Services::find(&service_token.unwrap().payload().service);
    if o_service.is_none() {
        return Err((StatusCode::UNAUTHORIZED, String::from("Service not found")));
    }
    
    return Ok(o_service.unwrap());
}