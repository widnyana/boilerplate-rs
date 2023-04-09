use aide::{
	axum::{routing::post_with, ApiRouter},
	transform::TransformOperation,
};
use infrastructure::openapi::openapi_tag;

pub fn router<AppState>() -> ApiRouter<AppState> {
	// let tag = openapi_tag("User");
	ApiRouter::new()
	// .api_route_with(
	//     "/api/users",
	//     post_with(dashboard_access, dashboard_access_operation),
	//     &tag,
	// )
	// .api_route_with(
	//     "/auth/logout/",
	//     post_with(api_not_implemented, logout_operation),
	//     &tag,
	// )
	// .api_route_with(
	//     "/auth/app-portal-access/:app_id/",
	//     post_with(app_portal_access, app_portal_access_operation),
	//     tag,
	// )
}
