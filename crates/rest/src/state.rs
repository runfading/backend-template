use application::service::demo::service::DemoApplicationService;
use std::sync::Arc;

/// Axum-specific dependencies exposed to REST handlers.
#[derive(Clone)]
pub struct AppState {
    pub demo_service: Arc<dyn DemoApplicationService>,
}
