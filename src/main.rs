mod infrastructure {
    pub mod service {
        pub mod Service;
    }
    pub mod controller {
        pub mod Controller;
        pub mod Handler;
        pub mod Utils;
    }
    pub mod dto {
        pub mod DtoPubKeyRequest;
        pub mod DtoSymetricKey;
        pub mod DtoSuscribePayload;
        pub mod DtoService;
        pub mod DtoPubKeyResponse;
    }
    pub mod client {
        pub mod HeaderParameter;
        pub mod QueryParameter;
        pub mod CryptoRequest;
        pub mod CryptoResponse;
        pub mod CryptoClient;
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
            pub mod symmetric {
                pub mod Utils;
                pub mod SymmetricManager;
                pub mod SymmetricKeys;
                pub mod SymmetricKey;
                pub mod AesBytes;
                pub mod Aes;
                pub mod AesGcm;
                pub mod AesGcmMessage;
            }
            pub mod asymmetric {
                pub mod Utils;
                pub mod AsymmetricManager;
                pub mod AsymmetricKeys;
                pub mod AsymmetricPrivate;
                pub mod AsymmetricPublic;
                pub mod Rsa;
            }
        }
    }
    pub mod exception {
        pub mod ErrorCodes;
        pub mod AuthenticationAppException;
        pub mod AuthenticationApiException;
    }
}

mod domain {
    pub mod Services;
    pub mod Service;
    pub mod PassToken;
}

use std::net::SocketAddr;

use axum::Router;
use commons::configuration::Configurator;
use infrastructure::controller::Controller;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let result = Configurator::initialize(); 
    if result.is_err() {
        println!("Configuration error: {}", result.err().unwrap().message());
        return;
    }

    let router = Router::new();
    let app = Controller::route(router).into_make_service_with_connect_info::<SocketAddr>();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}