use crate::common::{AppState, RouteRegistrar};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

mod handler;
pub mod models;
mod repo;
mod service;

#[derive(OpenApi)]
#[openapi(tags(
    (name = "demo", description = "demo"),
))]
pub struct BlogDoc;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(BlogDoc::openapi()).nest(
        "/demo",
        OpenApiRouter::new().routes(routes!(handler::latest_articles,)),
    )
}

inventory::submit!(RouteRegistrar { routes_fn: routes });
