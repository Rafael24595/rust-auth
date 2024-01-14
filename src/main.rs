mod infrastructure {
    pub mod controller;
    pub mod Service;
    pub mod DtoPubKeyRequest;
    pub mod DtoService;
    pub mod DtoPubKeyResponse;
    pub mod entity {
        pub mod HeaderParameter;
        pub mod QueryParameter;
        pub mod CryptoRequest;
    }
}

mod commons {
    pub mod configuration {
        pub mod Configurator;
        pub mod Configuration;
    }
    pub mod crypto {
        pub mod CryptoConfiguration;
        pub mod ServiceToken;
        pub mod Payload;
        pub mod modules {
            pub mod CryptoManager;
            pub mod Rsa;
        }
    }
    pub mod exception {
        pub mod AuthenticationApiException;
    }
}

mod domain {
    pub mod Services;
    pub mod Service;
    pub mod Key;
    pub mod PassToken;
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