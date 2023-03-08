use axum::{http::Method, routing::get, Router};
use std::{net::SocketAddr, sync::Arc};
use tracing::Level;

use tower_http::{
    add_extension::AddExtensionLayer,
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


mod shared;

#[tokio::main]
async fn main() {
    let state = shared::State {};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_print_request_response=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    // CORS
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);

    let svc = Router::new()
        .route("/", get(root))
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                // .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO).latency_unit(LatencyUnit::Micros)),
        )
        .layer(AddExtensionLayer::new(Arc::new(state)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr).serve(svc.into_make_service()).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
