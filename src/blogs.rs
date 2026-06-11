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
    (name = "blogs", description = "博客相关接口"),
))]
pub struct BlogDoc;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(BlogDoc::openapi()).nest(
        "/blogs",
        OpenApiRouter::new()
            .routes(routes!(
                handler::get_article,
                handler::create_article,
                handler::update_article,
                handler::delete_article,
            ))
            .routes(routes!(handler::latest_articles, handler::upsert,))
            .routes(routes!(handler::list_articles,)),
    )
}

inventory::submit!(RouteRegistrar { routes_fn: routes });
