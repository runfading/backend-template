use crate::error::ApiResult;
use crate::handlers::{AppState, PublicRouteRegistrar};
use api::ApiResponse;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[derive(OpenApi)]
#[openapi(tags(
    (name = "auth", description = "authentication"),
))]
struct AuthDoc;

pub fn public_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(AuthDoc::openapi())
        .nest("/auth", OpenApiRouter::new().routes(routes!(login)))
}

inventory::submit!(PublicRouteRegistrar {
    routes_fn: public_routes
});

#[derive(Debug, Deserialize, ToSchema)]
struct LoginRequest {
    /// Temporary user id. The user is assumed to exist until user storage is integrated.
    user_id: i64,
}

#[derive(Debug, Serialize, ToSchema)]
struct LoginResponse {
    access_token: String,
    token_type: &'static str,
    expires_in: u64,
}

#[utoipa::path(
    post,
    path = "/login",
    tag = "auth",
    security(()),
    request_body = LoginRequest,
    responses((status = 200, body = ApiResponse<LoginResponse>))
)]
async fn login(
    State(state): State<AppState>,
    axum::Json(input): axum::Json<LoginRequest>,
) -> ApiResult<LoginResponse> {
    let access_token = state.jwt().issue_token(input.user_id)?;

    Ok(ApiResponse::ok(LoginResponse {
        access_token,
        token_type: "Bearer",
        expires_in: state.jwt().access_token_ttl_seconds(),
    }))
}
