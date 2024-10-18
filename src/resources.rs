//! provides the [`Resources`] struct.
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::error::ResourceError;

#[derive(Default)]
pub struct Resources {
    data: RwLock<HashMap<TypeId, Arc<RwLock<dyn Any + Send + Sync>>>>,
}

impl Resources {
    pub(crate) fn add(&self, data: impl Any + Send + Sync) {
        self.data
            .write()
            .unwrap()
            .insert(data.type_id(), Arc::new(RwLock::new(data)));
    }

    pub(crate) fn resource_ref<T: Any + Send + Sync, R: FnOnce(&T)>(
        &self,
        run: R,
    ) -> Result<(), ResourceError> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.read().unwrap().get(&type_id) {
            run(data.read().unwrap().downcast_ref().unwrap());
            Ok(())
        } else {
            Err(ResourceError::ResourceDoesNotExist)
        }
    }

    pub(crate) fn resource_mut<T: Any + Send + Sync, R: FnOnce(&mut T)>(
        &self,
        run: R,
    ) -> Result<(), ResourceError> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.read().unwrap().get(&type_id) {
            run(data.write().unwrap().downcast_mut().unwrap());
            Ok(())
        } else {
            Err(ResourceError::ResourceDoesNotExist)
        }
    }

    pub(crate) fn remove<T: Any>(&self) {
        let type_id = TypeId::of::<T>();
        self.data.write().unwrap().remove(&type_id);
    }
}

#[cfg(test)]
mod test {
    use super::Resources;

    #[test]
    fn get_resource() {
        let resources = Resources::default();
        resources.add(32_u32);
        resources.resource_mut(|n: &mut u32| *n += 1).unwrap();
        resources
            .resource_ref(|n: &u32| assert_eq!(*n, 33))
            .unwrap();
    }
}
