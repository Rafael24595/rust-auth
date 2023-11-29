mod infrastructure {
    pub mod controller;
}

use axum::Router;
use infrastructure::controller;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let mut app = Router::new();
    app = controller::route(app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}