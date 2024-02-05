use axum::{
    body::{to_bytes, Body}, extract::{Json, Path, Request}, http::StatusCode, middleware, response::{IntoResponse, Response}, routing::{get, post}, Router
};

use crate::{commons::{configuration::Configuration, exception::AuthenticationApiException}, infrastructure::{
    client::CryptoRequest, dto::{DtoPubKeyResponse, DtoSuscribePayload}, service::Service}};

use crate::domain::Service as WebService;

use super::{Handler, Utils};

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

async fn nodekey() -> Result<(StatusCode, Json<DtoPubKeyResponse::DtoPubKeyResponse>), impl IntoResponse> {
    let key = Service::nodekey().await;
    
    if key.is_ok() {
        return Ok((StatusCode::OK, Json(key.unwrap())));    
    }
    
    if key.is_err() {
        let error = key.err().unwrap();
        return Err(error.into_response());
    }

    let error = AuthenticationApiException::new(StatusCode::NOT_FOUND.as_u16(), String::from("Not found"));
    return Err(error.into_response());
}

async fn subscribe(Json(dto): Json<DtoSuscribePayload::DtoSuscribePayload>) -> Result<(StatusCode, String), impl IntoResponse> {
    let status = Service::subscribe(dto).await;

    if status.is_ok() {
        return Ok((StatusCode::OK, status.unwrap()));    
    }
    
    if status.is_err() {
        let error = status.err().unwrap();
        return Err(error.into_response());   
    }

    let error = AuthenticationApiException::new(StatusCode::NOT_FOUND.as_u16(), String::from("Not found"));
    return Err(error.into_response());
}

async fn renove(Json(dto): Json<DtoSuscribePayload::DtoSuscribePayload>) -> Result<(StatusCode, String), impl IntoResponse> {
    let status = Service::renove(dto).await;

    if status.is_ok() {
        return Ok((StatusCode::ACCEPTED, status.unwrap()));    
    }
    
    if status.is_err() {
        let error = status.err().unwrap();
        return Err(error.into_response());   
    }

    let error = AuthenticationApiException::new(StatusCode::NOT_FOUND.as_u16(), String::from("Not found"));
    return Err(error.into_response());
}

async fn status(Path(service): Path<String>) -> Result<(StatusCode, String), impl IntoResponse> {
    let status = Service::status(service).await;

    if status.is_ok() {
        return Ok((StatusCode::ACCEPTED, String::from("Service up.")));    
    }
    
    if status.is_err() {
        let error = status.err().unwrap();
        return Err(error.into_response());
    }

    let error = AuthenticationApiException::new(StatusCode::NOT_FOUND.as_u16(), String::from("Not found"));
    return Err(error.into_response());
}

async fn key(Path(service): Path<String>) -> Result<(StatusCode, Json<DtoPubKeyResponse::DtoPubKeyResponse>), impl IntoResponse> {
    let key = Service::key(service).await;

    if key.is_ok() {
        return Ok((StatusCode::ACCEPTED, Json(key.unwrap())));    
    }
    
    if key.is_err() {
        let error = key.err().unwrap();
        return Err(error.into_response());    
    }

    let error = AuthenticationApiException::new(StatusCode::NOT_FOUND.as_u16(), String::from("Not found"));
    return Err(error.into_response());
}

async fn resolve(Path((service, path)): Path<(String, String)>, request: Request) -> Result<Response<Body>, impl IntoResponse>  {
    let header = request.headers().get(String::from(Configuration::COOKIE_NAME));
    let o_web_service = Utils::token_service(header);
    if o_web_service.is_err() {
        let error = o_web_service.err().unwrap();
        return Err(error.into_response());
    }
    let web_service = o_web_service.unwrap();

    let crypto_request = parse_request(web_service.clone(), service, path, request).await;
    if crypto_request.is_err() {
        let error = crypto_request.err().unwrap();
        return Err(error.into_response());
    }

    let r_crypto_response = Service::resolve(crypto_request.unwrap()).await;
    if r_crypto_response.is_err() {
        let error = r_crypto_response.err().unwrap();
        return Err(error.into_response());
    }

    let response = Utils::parse_response(web_service, r_crypto_response.unwrap());
    if response.is_err() {
        let error = response.err().unwrap();
        return Err(error.into_response());
    }

    return Ok(response.unwrap());
}

async fn parse_request(web_service: WebService::Service, service: String, path: String, request: Request) -> Result<CryptoRequest::CryptoRequest, AuthenticationApiException::AuthenticationApiException>  {
    let method = request.method().to_string();
    let headers = request.headers().clone();
    let uri = request.uri().clone();
    let query = uri.query().unwrap_or_default();
    let mut body = Vec::new();
    let b_body = to_bytes(request.into_body(), usize::MAX).await;
    if b_body.is_ok() {
        if web_service.symetric_key().is_none() {
            return Err(AuthenticationApiException::new(StatusCode::FORBIDDEN.as_u16(), String::from("Symmetric key not found"))); 
        }
        let decrypted = web_service.symetric_key().unwrap().decrypt_message(&b_body.unwrap().to_vec());
        if decrypted.is_err() {
            return Err(AuthenticationApiException::new(StatusCode::FORBIDDEN.as_u16(), decrypted.err().unwrap().message())); 
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

    return Ok(crypto_request);
}