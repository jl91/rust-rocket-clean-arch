use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub mod entrypoints;
pub mod mappers;
pub mod repositories_impls;


pub mod dependency_injection {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    pub struct Container {
        instances: Mutex<HashMap<TypeId, Arc<dyn Any + Send>>>,
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
            instances.insert(TypeId::of::<T>(), Arc::new(instance));
        }

        pub fn register_boxed<T>(&self, instance: Box<T>)
        where
            T: 'static + Send,
        {
            let mut instances = self.instances.lock().unwrap();
            instances.insert(TypeId::of::<T>(), Arc::new(instance));
        }

        pub fn resolve<T>(&self) -> Option<Arc<T>>
        where
            T: 'static + Send,
        {
            let instances = self.instances.lock().unwrap();
            if let Some(instance) = instances.get(&TypeId::of::<T>()) {
                return Some(Arc::clone(instance).downcast().ok()?);
            }
            None
        }

        pub fn resolve_boxed<T>(&self) -> Option<Arc<Box<T>>>
        where
            T: 'static + Send,
        {
            let instances = self.instances.lock().unwrap();
            if let Some(instance) = instances.get(&TypeId::of::<T>()) {
                return Some(Arc::clone(instance).downcast().ok()?);
            }
            None
        }
    }
}


pub mod state {
    use std::sync::Arc;
    use rocket::request::FromRequest;
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