use crate::AppState;
use crate::error::ApiError;
use axum::extract::{FromRequestParts, Request, State};
use axum::http::{header, request::Parts};
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct JwtConfig {
    secret: Arc<str>,
    access_token_ttl_seconds: u64,
}

impl JwtConfig {
    pub fn new(secret: impl Into<String>, access_token_ttl_seconds: u64) -> Self {
        let secret = secret.into();
        assert!(!secret.is_empty(), "JWT secret must not be empty");
        assert!(
            access_token_ttl_seconds > 0,
            "access token TTL must be greater than zero"
        );
        Self {
            secret: Arc::from(secret),
            access_token_ttl_seconds,
        }
    }

    pub fn access_token_ttl_seconds(&self) -> u64 {
        self.access_token_ttl_seconds
    }

    pub(crate) fn issue_token(&self, user_id: i64) -> Result<String, ApiError> {
        let now = unix_timestamp();
        let claims = Claims {
            sub: user_id,
            iat: now,
            exp: now.saturating_add(self.access_token_ttl_seconds),
        };

        jsonwebtoken::encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(ApiError::TokenCreation)
    }

    fn verify_token(&self, token: &str) -> Result<Claims, ApiError> {
        let validation = Validation::new(Algorithm::HS256);
        jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|_| ApiError::Unauthorized("invalid or expired access token"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentUser {
    pub user_id: i64,
}

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<CurrentUser>()
            .copied()
            .ok_or(ApiError::Unauthorized("authentication is required"))
    }
}

/// 请求权
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    sub: i64,
    iat: u64,
    exp: u64,
}

/// token校验并把数据放入参数里
/// 可以通过下面的方式获取
/// ```rust
///  pub async fn create(
///       current_user: CurrentUser,
///       State(state): State<AppState>,
///   ) -> ApiResult<DemoDTO> {}
/// ```
pub(crate) async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|token| !token.is_empty())
        .ok_or(ApiError::Unauthorized("missing bearer access token"))?;

    let claims = state.jwt().verify_token(token)?;
    request.extensions_mut().insert(CurrentUser {
        user_id: claims.sub,
    });

    Ok(next.run(request).await)
}

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::{CurrentUser, JwtConfig, require_auth};
    use crate::AppState;
    use axum::Router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode, header};
    use axum::middleware;
    use axum::routing::get;
    use sea_orm::DatabaseConnection;
    use tower::ServiceExt;

    fn state() -> AppState {
        AppState::new(
            DatabaseConnection::default(),
            JwtConfig::new("test-secret", 3600),
        )
    }

    fn protected_router(state: AppState) -> Router {
        Router::new()
            .route(
                "/protected",
                get(|user: CurrentUser| async move { user.user_id.to_string() }),
            )
            .route_layer(middleware::from_fn_with_state(state, require_auth))
    }

    #[tokio::test]
    async fn rejects_request_without_token() {
        let response = protected_router(state())
            .oneshot(Request::get("/protected").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn accepts_valid_token_and_sets_current_user() {
        let state = state();
        let token = state.jwt().issue_token(42).unwrap();
        let response = protected_router(state)
            .oneshot(
                Request::get("/protected")
                    .header(header::AUTHORIZATION, format!("Bearer {token}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
