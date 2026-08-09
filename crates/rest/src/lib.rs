{% if auth %}pub mod auth;
{% endif %}pub mod error;
pub mod handlers;
pub mod state;

{%- if auth %}
pub use auth::{CurrentUser, JwtConfig};
{%- endif %}
pub use handlers::{RouterConfig, init_router};
pub use state::AppState;
