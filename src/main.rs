mod infrastructure {
    pub mod controller;
}

mod commons {
    pub mod configuration {
        pub mod Configurator;
    }
}

mod domain {
    pub mod Auth;
    pub mod Service;
    pub mod Key;
}

use axum::Router;
use commons::configuration::Configurator;
use infrastructure::controller;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    Configurator::initialize();

    let mut app = Router::new();
    app = controller::route(app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}