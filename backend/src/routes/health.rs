// use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};

use std::sync::Arc;

use aide::axum::{routing::get, ApiRouter};
use axum::{http::StatusCode, Extension, Json};
use infrastructure::{context::AppContext, openapi::openapi_tag};
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatusVariant {
	Ok,
	Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthStatus {
	status: HealthStatusVariant,
}

impl HealthStatus {
	pub fn is_ok(&self) -> bool {
		matches!(self, HealthStatus { status: HealthStatusVariant::Ok })
	}

	pub fn new_ok() -> HealthStatus {
		HealthStatus { status: HealthStatusVariant::Ok }
	}

	pub fn new_error() -> HealthStatus {
		HealthStatus { status: HealthStatusVariant::Error }
	}
}

impl<O, E> From<Result<O, E>> for HealthStatus {
	fn from(res: Result<O, E>) -> Self {
		match res {
			Ok(_) => HealthStatus::new_ok(),
			Err(_) => HealthStatus::new_error(),
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthReport {
	database: HealthStatus,
}

async fn healthcheck(state: Extension<Arc<AppContext>>) -> (StatusCode, Json<HealthReport>) {
	// pub async fn healthcheck() -> impl IntoResponse {

	// let database = HealthStatus {
	//     status: HealthStatusVariant::Ok,
	// }

	let database: HealthStatus = state.db.execute(Statement::from_string(DatabaseBackend::Postgres, "SELECT 1".to_owned())).await.into();

	// let status = StatusCode::OK;

	let status = if database.is_ok() { StatusCode::OK } else { StatusCode::INTERNAL_SERVER_ERROR };

	let resp = HealthReport { database };

	(status, Json(resp))
}

async fn ping() -> StatusCode {
	StatusCode::NO_CONTENT
}

pub fn router() -> ApiRouter {
	let tag = openapi_tag("Utility");

	ApiRouter::new()
		.api_route_with("/health", get(healthcheck).head(healthcheck), &tag)
		.api_route_with("/health/ping", get(ping).head(ping), &tag)
}
