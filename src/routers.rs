use crate::common::{AppState, RouteRegistrar};
use axum::Router;
use axum::extract::Request;
use axum::http::HeaderName;
use tower_http::request_id::{
    MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{Level, info_span};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

pub fn init_router(state: AppState) -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|req: &Request<_>| {
            let request_id = req
                .extensions()
                .get::<RequestId>()
                .and_then(|id| id.header_value().to_str().ok())
                .unwrap_or("unknown");

            info_span!(
                "request",
                request_id = %request_id,
                method = %req.method(),
                uri = %req.uri(),
            )
        })
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    Router::new()
        .merge(routers())
        .with_state(state)
        .layer(trace_layer)
        .layer(PropagateRequestIdLayer::new(HeaderName::from_static(
            "x-request-id",
        )))
        .layer(SetRequestIdLayer::new(
            HeaderName::from_static("x-request-id"),
            MakeRequestUuid,
        ))
}

fn swagger_router(api: OpenApiRouter<AppState>) -> Router<AppState> {
    let (router, api) = api.split_for_parts();
    router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api))
}

pub fn routers() -> Router<AppState> {
    let mut router = OpenApiRouter::new();
    for registrar in inventory::iter::<RouteRegistrar> {
        router = router.merge((registrar.routes_fn)());
    }
    let router = OpenApiRouter::from(swagger_router(router));
    Router::new().merge(router)
}
