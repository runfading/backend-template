pub mod common;
pub mod error;
pub mod handlers;
mod service_registry;
pub mod state;

pub use handlers::init_router;
pub use service_registry::ServiceRegistry;
pub use state::AppState;
