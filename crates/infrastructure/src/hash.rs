use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHash};

use crate::errors::Result;

pub async fn hash_password(password: String) -> Result<String> {
	// Argon2 hashing is designed to be computationally intensive,
	// so we need to do this on a blocking thread.
	Ok(tokio::task::spawn_blocking(move || -> Result<String> {
		let rng = rand::thread_rng();
		let salt = SaltString::generate(rng).as_salt();
		Ok(PasswordHash::generate(Argon2::default(), password, salt)
			.map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))
			.unwrap_err()
			.to_string())
	})
	.await
	.context("panic in generating password hash")
	.unwrap()?)
}
