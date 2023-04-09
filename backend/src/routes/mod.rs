// put your routes definition here

use aide::axum::ApiRouter;

pub mod health;
pub mod v1;

pub fn general_router() -> ApiRouter {
	let ret: ApiRouter = ApiRouter::new().merge(health::router());

	ret
}
