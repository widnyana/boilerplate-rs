#![forbid(unsafe_code)]
#![allow(clippy::new_without_default)]
#![deny(rust_2018_idioms)]
#![allow(dead_code)]

use std::{process::ExitCode, sync::Arc};

use axum::{extract::Extension, http::Method, routing::get, Router, Server};
use backend::shared::shutdown_signal;
use infrastructure::{
	config::{get_config, Config},
	context::AppContext,
	http::handler::{handle_404_json, health_handler, root_handler},
	tracing::{provide_trace_layer, setup_tracing},
};
use tower_http::{
	compression::CompressionLayer,
	cors::{Any, CorsLayer},
};
use tracing::{info, instrument};

#[instrument(skip(ctx))]
pub(crate) fn make_app(ctx: Arc<AppContext>) -> Router {
	let cors = CorsLayer::new().allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE]).allow_origin(Any).allow_headers(Any);

	Router::new()
		.route("/", get(root_handler))
		.route("/health", get(health_handler))
		.layer(CompressionLayer::new())
		.layer(Extension(ctx))
		.layer(cors)
		.layer(provide_trace_layer())
		.fallback(handle_404_json)
}

#[instrument]
async fn start(config: &'static Config) -> Result<(), Box<dyn std::error::Error>> {
	let ctx: Arc<AppContext> = Arc::new(AppContext::init(config).await?);

	let addr = &format!("0.0.0.0:{}", config.port).parse().expect("Unable to parse bind address");

	let service: Router<()> = make_app(ctx);

	info!("serving on {addr}");

	Server::bind(addr).serve(service.into_make_service()).with_graceful_shutdown(shutdown_signal()).await?;

	Ok(())
}

#[tokio::main]
#[instrument]
async fn main() -> ExitCode {
	let config = get_config();
	setup_tracing(&config.log);

	match start(config).await {
		Ok(_) => ExitCode::SUCCESS,
		Err(err) => {
			eprintln!("Error: {err}");
			ExitCode::FAILURE
		}
	}
}
