use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

use serde::{Deserialize, Serialize};

pub fn route(router: Router) -> Router {
    return router.route("/", get(root))
        .route("/users", post(create_user));
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 8080,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}