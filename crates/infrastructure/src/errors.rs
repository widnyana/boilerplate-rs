// mostly stolen from svix-webhooks/server/svix-server/src/error.rs

use std::{
	error,
	fmt,
	fmt::{Debug, Formatter},
};

use aide::OperationOutput;
use axum::{
	extract::rejection::{ExtensionRejection, PathRejection, TypedHeaderRejection},
	headers::{authorization::Bearer, Authorization},
	response::{IntoResponse, Response},
	Json,
	TypedHeader,
};
use axum_jsonschema::JsonSchemaRejection;
use http::StatusCode;
use schemars::{JsonSchema, _serde_json::json};
use sea_orm::{DbErr, TransactionError};
use serde::Serialize;
use serde_json;

/// A short-hand version of a [std::result::Result] that always returns an
/// [Error].
pub type Result<T> = std::result::Result<T, Error>;

/// =============================================================================================
/// Generic Error
/// =============================================================================================

#[derive(Debug, Clone)]
pub enum ErrorType {
	/// A generic error
	Generic(String),
	/// Database error
	Database(String),
	/// Queue error
	Queue(String),
	/// Database error
	Validation(String),
	/// Any kind of HttpError
	Http(HttpError),
	/// Cache error
	Cache(String),
	/// Timeout error
	Timeout(String),
}

/// The error type returned from the Svix API
#[derive(Debug)]
pub struct Error {
	// the file name and line number of the error. Used for debugging non Http errors
	pub trace: Vec<&'static str>,
	pub typ: ErrorType,
}

/// implementation for [`Error`]
impl Error {
	pub fn new(s: impl fmt::Display, location: &'static str) -> Self {
		Self {
			trace: Self::init_trace(location),
			typ: ErrorType::Generic(s.to_string()),
		}
	}

	pub fn generic(s: impl fmt::Display, location: &'static str) -> Self {
		Self {
			trace: Self::init_trace(location),
			typ: ErrorType::Generic(s.to_string()),
		}
	}

	pub fn database(s: impl fmt::Display, location: &'static str) -> Self {
		Self {
			trace: Self::init_trace(location),
			typ: ErrorType::Database(s.to_string()),
		}
	}

	pub fn queue(s: impl fmt::Display, location: &'static str) -> Self {
		Self {
			trace: Self::init_trace(location),
			typ: ErrorType::Queue(s.to_string()),
		}
	}

	pub fn validation(s: impl fmt::Display, location: &'static str) -> Self {
		Self {
			trace: Self::init_trace(location),
			typ: ErrorType::Validation(s.to_string()),
		}
	}

	pub fn http(h: HttpError) -> Self {
		Self {
			trace: vec![], // no debugging necessary
			typ: ErrorType::Http(h),
		}
	}

	pub fn cache(s: impl fmt::Display, location: &'static str) -> Self {
		Self {
			trace: Self::init_trace(location),
			typ: ErrorType::Cache(s.to_string()),
		}
	}

	fn init_trace(location: &'static str) -> Vec<&'static str> {
		let mut trace = Vec::with_capacity(10); // somewhat arbitrary capacity, but avoids reallocation when building an error
										// trace later on
		trace.push(location);
		trace
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.typ.fmt(f)
	}
}

impl error::Error for Error {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		None
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		match self.typ {
			ErrorType::Http(s) => {
				tracing::debug!("{:?}, location: {:?}", &s, &self.trace);
				s.into_response()
			}
			s => {
				tracing::error!("type: {:?}, location: {:?}", s, &self.trace);
				(StatusCode::INTERNAL_SERVER_ERROR, Json(json!({}))).into_response()
			}
		}
	}
}

impl OperationOutput for Error {
	type Inner = Self;
}

/// ================================================================================================
/// Validation Error
/// ================================================================================================

/// Validation errors have their own schema to provide context for invalid
/// requests eg. mismatched types and out of bounds values. There may be any
/// number of these per 422 UNPROCESSABLE ENTITY error.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, JsonSchema)]
pub struct ValidationErrorItem {
	/// The location as a [`Vec`] of [`String`]s -- often in the form `["body",
	/// "field_name"]`, `["query", "field_name"]`, etc. They may, however, be
	/// arbitarily deep.
	pub loc: Vec<String>,

	/// The message accompanying the validation error item.
	pub msg: String,

	/// The type of error, often "type_error" or "value_error", but sometimes
	/// with more context like as "value_error.number.not_ge"
	#[serde(rename = "type")]
	pub ty: String,
}

/// ================================================================================================
/// HTTP Error Section
/// ================================================================================================

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum HttpErrorBody {
	Standard { code: String, detail: String },
	Validation { detail: Vec<ValidationErrorItem> },
}

#[derive(Debug, Clone)]
pub struct HttpError {
	pub status: StatusCode,
	body: HttpErrorBody,
}

impl HttpError {
	fn new_standard(status: StatusCode, code: String, detail: String) -> Self {
		Self {
			status,
			body: HttpErrorBody::Standard { code, detail },
		}
	}
	pub fn bad_request(code: Option<String>, detail: Option<String>) -> Self {
		Self::new_standard(
			StatusCode::BAD_REQUEST,
			code.unwrap_or_else(|| "generic_error".to_owned()),
			detail.unwrap_or_else(|| "Generic error".to_owned()),
		)
	}

	pub fn not_found(code: Option<String>, detail: Option<String>) -> Self {
		Self::new_standard(
			StatusCode::NOT_FOUND,
			code.unwrap_or_else(|| "not_found".to_owned()),
			detail.unwrap_or_else(|| "Entity not found".to_owned()),
		)
	}

	pub fn unauthorized(code: Option<String>, detail: Option<String>) -> Self {
		Self::new_standard(
			StatusCode::UNAUTHORIZED,
			code.unwrap_or_else(|| "authentication_failed".to_owned()),
			detail.unwrap_or_else(|| "Incorrect authentication credentials.".to_owned()),
		)
	}

	pub fn permission_denied(code: Option<String>, detail: Option<String>) -> Self {
		Self::new_standard(
			StatusCode::FORBIDDEN,
			code.unwrap_or_else(|| "insufficient access".to_owned()),
			detail.unwrap_or_else(|| "Insufficient access for the given operation.".to_owned()),
		)
	}

	pub fn conflict(code: Option<String>, detail: Option<String>) -> Self {
		Self::new_standard(
			StatusCode::CONFLICT,
			code.unwrap_or_else(|| "conflict".to_owned()),
			detail.unwrap_or_else(|| "A conflict has occurred".to_owned()),
		)
	}

	pub fn unprocessable_entity(detail: Vec<ValidationErrorItem>) -> Self {
		Self {
			status: StatusCode::UNPROCESSABLE_ENTITY,
			body: HttpErrorBody::Validation { detail },
		}
	}

	pub fn internal_server_error(code: Option<String>, detail: Option<String>) -> Self {
		Self::new_standard(
			StatusCode::INTERNAL_SERVER_ERROR,
			code.unwrap_or_else(|| "server_error".to_owned()),
			detail.unwrap_or_else(|| "Internal Server Error".to_owned()),
		)
	}

	pub fn not_implemented(code: Option<String>, detail: Option<String>) -> Self {
		Self::new_standard(
			StatusCode::NOT_IMPLEMENTED,
			code.unwrap_or_else(|| "not_implemented".to_owned()),
			detail.unwrap_or_else(|| "This API endpoint is not yet implemented.".to_owned()),
		)
	}
}

impl From<HttpError> for Error {
	fn from(err: HttpError) -> Error {
		Error::http(err)
	}
}

impl IntoResponse for HttpError {
	fn into_response(self) -> Response {
		(self.status, Json(self.body)).into_response()
	}
}

impl fmt::Display for HttpError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match &self.body {
			HttpErrorBody::Standard { code, detail } => write!(f, "status={} code=\"{}\" detail=\"{}\"", self.status, code, detail),

			HttpErrorBody::Validation { detail } => {
				write!(
					f,
					"status={} detail={}",
					self.status,
					serde_json::to_string(&detail).unwrap_or_else(|e| format!("\"unserializable error for {e}\""))
				)
			}
		}
	}
}

impl From<JsonSchemaRejection> for HttpError {
	fn from(rejection: JsonSchemaRejection) -> Self {
		match rejection {
			JsonSchemaRejection::Json(j) => Self::internal_server_error(None, Option::from(j.to_string())),
			JsonSchemaRejection::Serde(_) => Self::bad_request(None, Option::from("invalid request".to_string())),
			JsonSchemaRejection::Schema(_s) => Self::bad_request(None, Option::from("invalid request".to_string())),
		}
	}
}

/// ================================================================================================

pub trait Traceable<T> {
	fn trace(self, location: &'static str) -> Result<T>;
}

/// Adds [location!] data to the given result, returning a
/// [crate::error::Result<T>].
#[macro_export]
macro_rules! ctx {
	($res:expr) => {
		$crate::error::Traceable::trace($res, $crate::location!())
	};
}

impl<T> Traceable<T> for Result<T> {
	fn trace(self, location: &'static str) -> Result<T> {
		self.map_err(|mut e| {
			e.trace.push(location);
			e
		})
	}
}

impl<T> Traceable<T> for std::result::Result<T, DbErr> {
	fn trace(self, location: &'static str) -> Result<T> {
		self.map_err(|e| Error::database(e, location))
	}
}

impl<T> Traceable<T> for std::result::Result<T, redis::RedisError> {
	fn trace(self, location: &'static str) -> Result<T> {
		self.map_err(|e| Error::queue(e, location))
	}
}

impl<T> Traceable<T> for std::result::Result<T, ExtensionRejection> {
	fn trace(self, location: &'static str) -> Result<T> {
		self.map_err(|e| Error::generic(e, location))
	}
}

impl<T> Traceable<T> for std::result::Result<T, PathRejection> {
	fn trace(self, location: &'static str) -> Result<T> {
		self.map_err(|e| Error::generic(e, location))
	}
}

impl Traceable<TypedHeader<Authorization<Bearer>>> for std::result::Result<TypedHeader<Authorization<Bearer>>, TypedHeaderRejection> {
	fn trace(self, _location: &'static str) -> Result<TypedHeader<Authorization<Bearer>>> {
		self.map_err(|_err| HttpError::unauthorized(None, Some("Invalid token".to_string())).into())
	}
}

impl<T> Traceable<T> for std::result::Result<T, TransactionError<Error>> {
	fn trace(self, location: &'static str) -> Result<T> {
		self.map_err(|e| match e {
			TransactionError::Connection(db_err) => Error::database(db_err, location),
			TransactionError::Transaction(crate_err) => crate_err, // preserve the trace that comes from within the transaction
		})
	}
}
