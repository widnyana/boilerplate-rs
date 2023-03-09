use axum::{
    http::Method,
    middleware::{self},
    routing::get,
    Router,
};
use slog::{o, Drain};

use std::{net::SocketAddr, sync::Arc, sync::Mutex};
use tower_http::{
    add_extension::AddExtensionLayer,
    cors::{Any, CorsLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use backend::shared;
use infrastructure::{info, set_global};

#[tokio::main]
async fn main() {
    let state = shared::State {};
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let logger = slog::Logger::root(
        Mutex::new(slog_json::Json::default(std::io::stderr())).map(slog::Fuse),
        o!("version" => env!("CARGO_PKG_VERSION")),
    );

    set_global(logger);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "example_print_request_response=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // CORS
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);

    let svc = Router::new()
        .route("/", get(root))
        .fallback(shared::handler_404)
        .layer(middleware::from_fn(shared::print_request_response))
        .layer(cors)
        .layer(AddExtensionLayer::new(Arc::new(state)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("http server is running"; "bind_addr" => addr);
    axum::Server::bind(&addr)
        .serve(svc.into_make_service())
        .with_graceful_shutdown(shared::shutdown_signal())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
