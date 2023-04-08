use async_trait::async_trait;
use sea_orm::ConnectionTrait;
use serde::{Deserialize, Serialize};

use crate::result::Result;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DeleteEntity {
	pub entity_name: String,
	pub record_id: String,
	pub content: String,
}

#[async_trait]
pub trait SeaORMExtend {
	async fn soft_delete<C>(self, delete_user: &str, db: &C) -> Result<u64>
	where
		C: ConnectionTrait;

	async fn soft_delete_with_pk<C>(self, custom_pk_field: &str, delete_user: &str, db: &C) -> Result<u64>
	where
		C: ConnectionTrait;

	async fn soft_delete_custom<C>(self, custom_pk_field: &str, db: &C) -> Result<Vec<DeleteEntity>>
	where
		C: ConnectionTrait;
}
