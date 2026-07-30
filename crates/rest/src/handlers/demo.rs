mod handler;
mod models;

use crate::handlers::{AppState, RouteRegistrar};
use application::service::demo::service::DemoApplicationService;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[derive(OpenApi)]
#[openapi(tags(
    (name = "demo", description = "demo"),
))]
pub struct DemoDoc;

pub fn routes(app_state: &AppState) -> OpenApiRouter<()> {
    OpenApiRouter::<DemoState>::with_openapi(DemoDoc::openapi())
        .nest(
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
        .with_state(DemoState::from_app_state(app_state))
}

inventory::submit!(RouteRegistrar { routes_fn: routes });

#[derive(Clone)]
pub struct DemoState {
    pub demo_service: Arc<dyn DemoApplicationService>,
}

impl DemoState {
    fn from_app_state(app_state: &AppState) -> Self {
        Self {
            demo_service: app_state.require::<Arc<dyn DemoApplicationService>>(),
        }
    }
}
