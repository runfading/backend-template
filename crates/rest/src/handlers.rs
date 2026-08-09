mod demo;

pub use crate::state::AppState;
use axum::Router;
use axum::extract::{DefaultBodyLimit, Request};
use axum::http::{HeaderName, HeaderValue, Method, StatusCode, header};
use std::time::Duration;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::request_id::{
    MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{Level, info_span};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

/// 路由设置
pub struct RouterConfig {
    pub request_timeout: Duration,
    pub max_body_size: usize,
    pub cors_origins: Vec<HeaderValue>,
}

/// 路由注册
pub struct RouteRegistrar {
    pub routes_fn: fn() -> OpenApiRouter<AppState>,
}

inventory::collect!(RouteRegistrar);

pub fn init_router(state: AppState, config: RouterConfig) -> Router {
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

    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::list(config.cors_origins))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true);

    Router::new()
        .merge(routers())
        .with_state(state)
        .layer(DefaultBodyLimit::max(config.max_body_size))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            config.request_timeout,
        ))
        .layer(trace_layer)
        .layer(PropagateRequestIdLayer::new(HeaderName::from_static(
            "x-request-id",
        )))
        .layer(SetRequestIdLayer::new(
            HeaderName::from_static("x-request-id"),
            MakeRequestUuid,
        ))
        .layer(cors_layer)
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

#[cfg(test)]
mod tests {
    use super::{RouterConfig, init_router};
    use crate::AppState;
    use axum::http::HeaderValue;
    use sea_orm::DatabaseConnection;
    use std::time::Duration;

    #[test]
    fn builds_router_from_shared_database_pool() {
        let state = AppState::new(DatabaseConnection::default());
        let config = RouterConfig {
            request_timeout: Duration::from_secs(30),
            max_body_size: 2 * 1024 * 1024,
            cors_origins: vec![HeaderValue::from_static("http://localhost:3000")],
        };
        let _router = init_router(state, config);
    }
}
