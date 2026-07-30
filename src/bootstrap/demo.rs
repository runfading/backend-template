use crate::bootstrap::ServiceRegistrar;
use application::service::demo::service::DemoApplicationService;
use application::service::demo::DemoService;
use infrastructure::dal::demo::SeaOrmDemoRepository;
use rest::ServiceRegistry;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

fn register(services: &mut ServiceRegistry, db_pool: &DatabaseConnection) {
    let repository = SeaOrmDemoRepository::new(db_pool.clone());
    let service: Arc<dyn DemoApplicationService> = Arc::new(DemoService::new(repository));
    services.insert(service);
}

// 服务注册
inventory::submit!(ServiceRegistrar {
    registry_fn: register
});
