use axum::{
    body::Body, extract::{Json, Path, Request}, http::{Response, StatusCode}, middleware, response::IntoResponse, routing::{get, post}, Router
};

use crate::{commons::{configuration::Configuration, exception::{AuthenticationApiException, ErrorCodes}}, infrastructure::{
    dto::{DtoExceptionData, DtoPubKeyResponse, DtoSuscribePayload}, service::Service}};

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

        .route("/exception/:exception", get(exception))
        
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

    return Err(not_found());
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

    return Err(not_found());
}

async fn renove(Json(dto): Json<DtoSuscribePayload::DtoSuscribePayload>) -> Result<(StatusCode, String), impl IntoResponse> {
    let status = Service::refresh_token(dto).await;

    if status.is_ok() {
        return Ok((StatusCode::ACCEPTED, status.unwrap()));    
    }
    
    if status.is_err() {
        let error = status.err().unwrap();
        return Err(error.into_response());   
    }

    return Err(not_found());
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

    return Err(not_found());
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

    return Err(not_found());
}

async fn resolve(Path((service, path)): Path<(String, String)>, request: Request) -> Result<Response<Body>, impl IntoResponse>  {
    let header = request.headers().get(String::from(Configuration::COOKIE_NAME));
    let o_web_service = Utils::token_service(header);
    if o_web_service.is_err() {
        let error = o_web_service.err().unwrap();
        return Err(error.into_response());
    }
    let web_service = o_web_service.unwrap();

    let crypto_request = Utils::parse_request(web_service.clone(), service, path, request).await;
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

async fn exception(Path(exception): Path<String>) -> Result<Json<DtoExceptionData::DtoExceptionData>, StatusCode> {
    let error = ErrorCodes::from_slice(exception);
    if error.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }
    return Ok(Json(error.unwrap().as_dto()));
}

fn not_found() -> Response<Body> {
    let error = AuthenticationApiException::new(
        StatusCode::INTERNAL_SERVER_ERROR.as_u16(), 
        ErrorCodes::ErrorCodes::SYSIN001,
        String::from("Not found"));
    return error.into_response();
}