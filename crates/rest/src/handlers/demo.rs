mod handler;
mod models;

use crate::handlers::{AppState, RouteRegistrar};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[derive(OpenApi)]
#[openapi(tags(
    (name = "demo", description = "demo"),
))]
pub struct DemoDoc;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(DemoDoc::openapi()).nest(
        "/demo",
        OpenApiRouter::new()
            // utoipa-axum 限制：同一个 routes! 宏内不允许出现重复的请求方法
            .routes(routes!(handler::list))
            .routes(routes!(handler::page))
            .routes(routes!(
                handler::get,
                handler::create,
                handler::update,
                handler::patch,
                handler::remove
            )),
    )
}

inventory::submit!(RouteRegistrar { routes_fn: routes });
