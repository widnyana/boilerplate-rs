use std::{
	convert::Infallible,
	fmt::{Display, Formatter},
	num::{ParseIntError, TryFromIntError},
	str::Utf8Error,
	string::FromUtf8Error,
	sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard},
};

use base64;
use hex::FromHexError;
use serde::{Deserialize, Serialize};
use tracing::log::warn;
use url::ParseError as URLParseError;

/// Chet unified error wrapper
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ChetError {
	pub code: String,
	pub message: String,
}

impl ChetError {
	fn error(code: &str, msg: &str) -> ChetError {
		warn!("[Chet.Error] {}:{}", code, msg);
		ChetError {
			code: code.to_string(),
			message: msg.to_string(),
		}
	}

	pub fn bad_request(msg: &str) -> ChetError {
		Self::error("400", msg)
	}

	pub fn conflict(msg: &str) -> ChetError {
		Self::error("409", msg)
	}

	pub fn format_error(msg: &str) -> ChetError {
		Self::error("406", msg)
	}

	pub fn io_error(msg: &str) -> ChetError {
		Self::error("503", msg)
	}

	pub fn internal_error(msg: &str) -> ChetError {
		Self::error("500", msg)
	}
}

impl Display for ChetError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "<{}> {}", self.code, self.message)
	}
}

impl From<std::io::Error> for ChetError {
	fn from(error: std::io::Error) -> Self {
		ChetError::io_error(&format!("[Chet.Basic] {error}"))
	}
}

impl From<Utf8Error> for ChetError {
	fn from(error: Utf8Error) -> Self {
		ChetError::format_error(&format!("[Tardis.Basic] {error}"))
	}
}

impl From<FromUtf8Error> for ChetError {
	fn from(error: FromUtf8Error) -> Self {
		ChetError::format_error(&format!("[Tardis.Basic] {error}"))
	}
}

impl From<URLParseError> for ChetError {
	fn from(error: URLParseError) -> Self {
		ChetError::format_error(&format!("[Tardis.Basic] {error}"))
	}
}

impl From<ParseIntError> for ChetError {
	fn from(error: ParseIntError) -> Self {
		ChetError::format_error(&format!("[Tardis.Basic] {error}"))
	}
}

impl From<Infallible> for ChetError {
	fn from(error: Infallible) -> Self {
		ChetError::format_error(&format!("[Tardis.Basic] {error}"))
	}
}

impl From<base64::DecodeError> for ChetError {
	fn from(error: base64::DecodeError) -> Self {
		ChetError::format_error(&format!("[Tardis.Basic] {error}"))
	}
}

impl From<FromHexError> for ChetError {
	fn from(error: FromHexError) -> Self {
		ChetError::format_error(&format!("[Tardis.Basic] {error}"))
	}
}

impl From<regex::Error> for ChetError {
	fn from(error: regex::Error) -> Self {
		ChetError::format_error(&format!("[Tardis.Basic] {error}"))
	}
}

impl From<TryFromIntError> for ChetError {
	fn from(error: TryFromIntError) -> Self {
		ChetError::format_error(&format!("[Tardis.Basic] {error}"))
	}
}

impl<P> From<PoisonError<RwLockReadGuard<'_, P>>> for ChetError {
	fn from(error: PoisonError<RwLockReadGuard<'_, P>>) -> Self {
		ChetError::conflict(&format!("[Tardis.Basic] {error}"))
	}
}

impl<P> From<PoisonError<RwLockWriteGuard<'_, P>>> for ChetError {
	fn from(error: PoisonError<RwLockWriteGuard<'_, P>>) -> Self {
		ChetError::conflict(&format!("[Tardis.Basic] {error}"))
	}
}
