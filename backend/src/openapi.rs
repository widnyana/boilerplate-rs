use std::sync::Arc;

use aide::{
	axum::{
		routing::{get, get_with},
		ApiRouter,
		IntoApiResponse,
	},
	openapi,
	openapi::OpenApi,
	redoc::Redoc,
	transform::TransformOpenApi,
};
use axum::{
	response::{Html, IntoResponse},
	Extension,
	Json,
};
use infrastructure::context::AppContext;

pub fn initialize_openapi() -> OpenApi {
	aide::gen::on_error(|error| {
		tracing::error!("Aide generation error: {error}");
	});
	// Extract schemas to `#/components/schemas/` instead of using inline schemas.
	aide::gen::extract_schemas(true);

	OpenApi {
		openapi: "",
		info: openapi::Info {
			title: env!("SERVICE_NAME").to_owned(),
			extensions: Default::default(),
			..Default::default()
		},
		tags: vec![],
		extensions: Default::default(),
		..Default::default()
	}
}

pub fn api_docs(api: TransformOpenApi<'_>) -> TransformOpenApi<'_> {
	api.title("such openapi, much docs, wow")
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
	Json(api).into_response()
}

async fn get_docs() -> Html<&'static str> {
	Html(include_str!("static/openapi.html"))
}

pub fn docs_routes(ctx: Arc<AppContext>) -> ApiRouter {
	aide::gen::infer_responses(true);

	let router = ApiRouter::new()
		.api_route_with(
			"/docs",
			get_with(
				Redoc::new("/docs/private/api.json").with_title("wololo titlte").axum_handler(),
				|op| op.description("a new description"),
			),
			|p| p.summary("a new summary"),
		)
		.route("/private/api.json", get(serve_docs))
		.api_route("/docsx", get(get_docs))
		.with_state(ctx);

	// Afterwards we disable response inference because
	// it might be incorrect for other routes.
	aide::gen::infer_responses(false);

	router
}
