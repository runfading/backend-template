use crate::common::{AppState, RouteRegistrar};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

mod handler;
pub mod models;
mod service;

#[derive(OpenApi)]
#[openapi(tags(
    (name = "demo", description = "demo"),
))]
pub struct DemoDoc;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(DemoDoc::openapi()).nest(
        "/demo",
        OpenApiRouter::new()
            // 同一种请求方式不能放一起
            .routes(routes!(handler::list))
            .routes(routes!(
                handler::get,
                handler::create,
                handler::update,
                handler::remove
            )),
    )
}

inventory::submit!(RouteRegistrar { routes_fn: routes });
