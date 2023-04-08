use axum::{http::StatusCode, response::IntoResponse, Json};
use tracing::{info, instrument};

use crate::http::response::PlainResponse;

/// Handle health check requests
#[instrument]
pub async fn health_handler() -> impl IntoResponse {
	let err = PlainResponse {
		error: false,
		message: "healthy".to_string(),
	};

	(StatusCode::OK, Json(err))
}

/// default response when 404 occured - as json
#[instrument]
pub async fn handle_404_json() -> impl IntoResponse {
	info!("got hit on handle_404_json");

	let err = PlainResponse {
		error: true,
		message: "not found".to_string(),
	};

	(StatusCode::NOT_FOUND, Json(err))
}

/// default response when 404 occured -  as plain text
pub async fn handler_404() -> impl IntoResponse {
	(StatusCode::NOT_FOUND, "nothing to see here")
}

/// old plain hello world
#[instrument]
pub async fn root_handler() -> impl IntoResponse {
	let idx = PlainResponse {
		error: false,
		message: "sup!".to_string(),
	};

	(StatusCode::OK, Json(idx))
}
