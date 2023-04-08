use serde::Serialize;

#[derive(Serialize)]
pub struct PlainResponse {
	pub error: bool,
	pub message: String,
}
