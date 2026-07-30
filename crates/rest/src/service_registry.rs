use std::any::{type_name, Any, TypeId};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// 动态注册容器，灵活，
/// 若出现类型错误在路由收集期间就可以被发现
#[derive(Default)]
pub struct ServiceRegistry {
    values: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T>(&mut self, service: T)
    where
        T: Send + Sync + 'static,
    {
        match self.values.entry(TypeId::of::<T>()) {
            Entry::Vacant(entry) => {
                entry.insert(Box::new(service));
            }
            Entry::Occupied(_) => {
                panic!("service already registered: {}", type_name::<T>());
            }
        }
    }

    pub fn get<T>(&self) -> Option<&T>
    where
        T: Send + Sync + 'static,
    {
        self.values
            .get(&TypeId::of::<T>())
            .and_then(|service| service.downcast_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::ServiceRegistry;

    #[test]
    fn stores_services_by_type() {
        let mut registry = ServiceRegistry::new();
        registry.insert(String::from("demo"));
        registry.insert(42_u32);

        assert_eq!(registry.get::<String>().map(String::as_str), Some("demo"));
        assert_eq!(registry.get::<u32>(), Some(&42));
    }

    #[test]
    #[should_panic(expected = "service already registered")]
    fn rejects_duplicate_service_types() {
        let mut registry = ServiceRegistry::new();
        registry.insert(1_u32);
        registry.insert(2_u32);
    }
}
