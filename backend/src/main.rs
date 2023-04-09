#![forbid(unsafe_code)]
#![allow(clippy::new_without_default)]
#![deny(rust_2018_idioms)]
#![allow(dead_code)]

use std::{process::ExitCode, sync::Arc};

use aide::axum::ApiRouter;
use axum::{extract::Extension, http::Method, routing::get, Server};
use backend::routes;
use infrastructure::{
	config::{get_config, Config},
	context::AppContext,
	http::handler::{handle_404_json, health_handler, root_handler},
	signal::graceful_shutdown_handler,
	tracing::{provide_trace_layer, setup_tracing},
};
use tower_http::{
	compression::CompressionLayer,
	cors::{Any, CorsLayer},
};
use tracing::instrument;

#[instrument(skip(ctx))]
pub fn make_app(ctx: Arc<AppContext>) -> ApiRouter {
	let cors = CorsLayer::new()
		.allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
		.allow_origin(Any)
		.allow_headers(Any);

	// initialize routes
	let general_router = routes::general_router();

	ApiRouter::new()
		.nest_api_service("/general", general_router)
		.route("/", get(root_handler))
		.route("/health", get(health_handler))
		.fallback(handle_404_json)
		.layer(CompressionLayer::new())
		.layer(Extension(ctx))
		.layer(cors)
		.layer(provide_trace_layer())
}

#[instrument(skip(config))]
async fn start(config: &'static Config) -> Result<(), Box<dyn std::error::Error>> {
	tracing::debug!("initializing application...");
	let ctx: Arc<AppContext> = Arc::new(AppContext::init(config).await?);
	let addr = &format!("0.0.0.0:{}", config.port).parse().expect("Unable to parse bind address");
	let service: ApiRouter<()> = make_app(ctx);

	tracing::info!("serving on {addr}");
	Server::bind(addr)
		.serve(service.into_make_service())
		.with_graceful_shutdown(graceful_shutdown_handler())
		.await?;

	Ok(())
}

#[tokio::main]
#[instrument]
async fn main() -> ExitCode {
	let config = get_config();
	setup_tracing(config);

	match start(config).await {
		Ok(_) => ExitCode::SUCCESS,
		Err(err) => {
			eprintln!("Error: {err}");
			ExitCode::FAILURE
		}
	}
}
