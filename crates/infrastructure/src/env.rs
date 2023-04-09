#![allow(clippy::all)]

use std::{net::SocketAddr, num::ParseIntError, string::String, time::Duration};

pub const HTTP_TIMEOUT: Duration = Duration::from_secs(5);

pub const CSS_MAX_AGE: Duration = Duration::from_secs(3600);

pub const FAVICON_MAX_AGE: Duration = Duration::from_secs(86400);

const VAR_ADDRESS_PORT: &str = "ENVCMP_ADDRESS_PORT";
const VAR_CACHE_SIZE: &str = "ENVCMP_CACHE_SIZE";
const VAR_DATABASE_URL: &str = "ENVCMP_DATABASE_URL";
const VAR_MAX_BODY_SIZE: &str = "ENVCMP_MAX_BODY_SIZE";
const VAR_SIGNING_KEY: &str = "ENVCMP_SIGNING_KEY";

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("failed to parse {VAR_CACHE_SIZE}, expected number of elements: {0}")]
	CacheSize(ParseIntError),
	#[error("failed to parse {VAR_DATABASE_URL}, contains non-Unicode data")]
	DatabasePath,
	#[error("failed to parse {VAR_MAX_BODY_SIZE}, expected number of bytes: {0}")]
	MaxBodySize(ParseIntError),
	#[error("failed to parse {VAR_ADDRESS_PORT}, expected `host:port`")]
	AddressPort,
	#[error("failed to generate key from {VAR_SIGNING_KEY}: {0}")]
	SigningKey(String),
}

pub fn addr() -> Result<SocketAddr, Error> {
	std::env::var(VAR_ADDRESS_PORT)
		.as_ref()
		.map(String::as_str)
		.unwrap_or("0.0.0.0:3000")
		.parse()
		.map_err(|_e| Error::AddressPort)
}

pub fn max_body_size() -> Result<usize, Error> {
	std::env::var(VAR_MAX_BODY_SIZE)
		.map_or_else(|_| Ok(1024 * 1024), |s| s.parse::<usize>())
		.map_err(Error::MaxBodySize)
}
