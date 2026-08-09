use crate::auth::JwtConfig;
use sea_orm::DatabaseConnection;

/// Shared infrastructure used to construct each business router state.
#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
    jwt: JwtConfig,
}

impl AppState {
    pub fn new(db: DatabaseConnection, jwt: JwtConfig) -> Self {
        Self { db, jwt }
    }

    pub(crate) fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub(crate) fn jwt(&self) -> &JwtConfig {
        &self.jwt
    }
}
