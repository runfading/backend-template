{% if auth %}use crate::auth::JwtConfig;
{% endif %}use sea_orm::DatabaseConnection;

/// Shared infrastructure used to construct each business router state.
#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
{%- if auth %}
    jwt: JwtConfig,
{%- endif %}
}

impl AppState {
{%- if auth %}
    pub fn new(db: DatabaseConnection, jwt: JwtConfig) -> Self {
        Self { db, jwt }
    }
{%- else %}
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
{%- endif %}

    pub(crate) fn db(&self) -> &DatabaseConnection {
        &self.db
    }

{%- if auth %}
    pub(crate) fn jwt(&self) -> &JwtConfig {
        &self.jwt
    }
{%- endif %}
}
