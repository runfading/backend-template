{% if auth %}mod auth;
{% endif %}mod demo;

{%- if auth %}
use crate::auth::require_auth;
{%- endif %}
pub use crate::state::AppState;
use axum::Router;
use axum::extract::{DefaultBodyLimit, Request};
use axum::http::{HeaderName, HeaderValue, Method, StatusCode, header};
{%- if auth %}
use axum::middleware;
{%- endif %}
use std::time::Duration;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::request_id::{
    MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{Level, info_span};
{%- if auth %}
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityRequirement, SecurityScheme};
{%- endif %}
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

/// Routes registered here explicitly bypass access-token validation.
pub struct PublicRouteRegistrar {
    pub routes_fn: fn() -> OpenApiRouter<AppState>,
}

inventory::collect!(RouteRegistrar);
inventory::collect!(PublicRouteRegistrar);

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
        .merge(routers(state.clone()))
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
{%- if auth %}
    let (router, mut api) = api.split_for_parts();
    let components = api.components.get_or_insert_default();
    components.add_security_scheme(
        "bearerAuth",
        SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
    );
    api.security = Some(vec![SecurityRequirement::new(
        "bearerAuth",
        Vec::<String>::new(),
    )]);
{%- else %}
    let (router, api) = api.split_for_parts();
{%- endif %}
    router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api))
}

pub fn routers(state: AppState) -> Router<AppState> {
    let mut protected_router = OpenApiRouter::new();
    for registrar in inventory::iter::<RouteRegistrar> {
        protected_router = protected_router.merge((registrar.routes_fn)());
    }

{%- if auth %}
    // 需要token校验的
    let protected_router =
        protected_router.route_layer(middleware::from_fn_with_state(state, require_auth));
{%- else %}
    let _ = state;
{%- endif %}

    // 无需校验的
    let mut public_router = OpenApiRouter::new();
    for registrar in inventory::iter::<PublicRouteRegistrar> {
        public_router = public_router.merge((registrar.routes_fn)());
    }

    let router = protected_router.merge(public_router);
    let router = OpenApiRouter::from(swagger_router(router));
    Router::new().merge(router)
}

#[cfg(test)]
mod tests {
    use super::{RouterConfig, init_router, routers};
    use crate::AppState;
{%- if auth %}
    use crate::JwtConfig;
{%- endif %}
    use axum::body::Body;
    use axum::http::HeaderValue;
    use axum::http::{Request, StatusCode};
    use sea_orm::DatabaseConnection;
    use std::time::Duration;
    use tower::ServiceExt;

    fn state() -> AppState {
{%- if auth %}
        AppState::new(
            DatabaseConnection::default(),
            JwtConfig::new("test-secret", 3600),
        )
{%- else %}
        AppState::new(DatabaseConnection::default())
{%- endif %}
    }

    #[test]
    fn builds_router_from_shared_database_pool() {
        let state = state();
        let config = RouterConfig {
            request_timeout: Duration::from_secs(30),
            max_body_size: 2 * 1024 * 1024,
            cors_origins: vec![HeaderValue::from_static("http://localhost:3000")],
        };
        let _router = init_router(state, config);
    }

{%- if auth %}
    #[tokio::test]
    async fn login_route_is_explicitly_public() {
        let state = state();
        let app = routers(state.clone()).with_state(state);
        let response = app
            .oneshot(
                Request::post("/auth/login")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"user_id":1}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn regular_routes_require_an_access_token_by_default() {
        let state = state();
        let app = routers(state.clone()).with_state(state);
        let response = app
            .oneshot(Request::get("/demo/list").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
{%- endif %}

    #[tokio::test]
    async fn openapi_document_is_explicitly_public() {
        let state = state();
        let app = routers(state.clone()).with_state(state);
        let response = app
            .oneshot(
                Request::get("/apidoc/openapi.json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
