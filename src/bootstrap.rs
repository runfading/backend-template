mod demo;

use rest::{AppState, ServiceRegistry};
use sea_orm::DatabaseConnection;

pub fn build_app_state(db_pool: DatabaseConnection) -> AppState {
    let mut services = ServiceRegistry::new();

    for registrar in inventory::iter::<ServiceRegistrar> {
        (registrar.registry_fn)(&mut services, &db_pool);
    }

    AppState::new(services)
}

/// 服务收集器
pub struct ServiceRegistrar {
    pub registry_fn: fn(registry: &mut ServiceRegistry, db_pool: &DatabaseConnection),
}

inventory::collect!(ServiceRegistrar);
