pub mod entrypoints;
pub mod mappers;
pub mod repositories_impls;

pub mod dependency_injection {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;
    use std::sync::{Mutex};

    pub struct Container {
        instances: Mutex<HashMap<TypeId, Box<dyn Any + Send>>>,
    }

    impl Container {
        pub fn new() -> Self {
            Container {
                instances: Mutex::new(HashMap::new()),
            }
        }

        pub fn register<T>(&self, instance: T)
            where
                T: 'static + Send,
        {
            let mut instances = self.instances.lock().unwrap();
            instances.insert(TypeId::of::<T>(), Box::new(instance));
        }

        pub fn resolve<T>(&self) -> Option<T>
            where
                T: 'static + Send + Clone,
        {
            let instances = self.instances.lock().unwrap();
            if let Some(instance) = instances.get(&TypeId::of::<T>()) {
                if let Some(instance) = instance.downcast_ref::<T>() {
                    return Some(instance.clone());
                }
            }
            None
        }
    }
}

pub mod state {
    use std::sync::Arc;
    use crate::application::dependency_injection::Container;

    pub struct AppState {
        pub di_container: Arc<Container>,
    }

    impl AppState {
        pub fn new() -> Self {
            AppState {
                di_container: Arc::new(Container::new()),
            }
        }
    }
}