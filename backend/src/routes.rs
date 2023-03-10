use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::shared::PlainResponse;

/// Handle health check requests
pub async fn health_handler() -> impl IntoResponse {
    let err = PlainResponse {
        error: false,
        message: "healthy".to_string(),
    };

    (StatusCode::OK, Json(err))
}

/// default response when 404 occured - as json
pub async fn handle_404_json() -> impl IntoResponse {
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
