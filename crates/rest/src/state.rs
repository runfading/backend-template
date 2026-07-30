use crate::service_registry::ServiceRegistry;
use std::any::type_name;
use std::sync::Arc;

/// Dependencies used by inventory route registrars.
#[derive(Clone)]
pub struct AppState {
    services: Arc<ServiceRegistry>,
}

impl AppState {
    pub fn new(services: ServiceRegistry) -> Self {
        Self {
            services: Arc::new(services),
        }
    }

    pub(crate) fn require<T>(&self) -> T
    where
        T: Clone + Send + Sync + 'static,
    {
        self.services
            .get::<T>()
            .unwrap_or_else(|| panic!("service not registered: {}", type_name::<T>()))
            .clone()
    }
}
