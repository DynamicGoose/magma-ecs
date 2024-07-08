use std::{
    any::{Any, TypeId}, sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::error::EntityError;

use super::Entities;

type ExtractedComponents<'a> = Result<&'a Vec<Option<Arc<RwLock<dyn Any + Send + Sync>>>>, EntityError>;

/// A query entity with the entities id and a reference to the `Entities` struct.
pub struct QueryEntity<'a> {
    pub id: usize,
    entities: &'a Entities,
}

impl<'a> QueryEntity<'a> {
    pub fn new(id: usize, entities: &'a Entities) -> Self {
        Self { id, entities }
    }

    fn extract_components<T: Any>(&self) -> ExtractedComponents {
        let type_id = TypeId::of::<T>();
        self.entities
            .components
            .get(&type_id)
            .ok_or(EntityError::ComponentNotInQuery)
    }
    pub fn get_component<T: Any>(&self) -> Result<RwLockReadGuard<dyn Any + Send + Sync>, EntityError> {
        let components = self.extract_components::<T>()?;
        let borrowed_component = components[self.id]
            .as_ref()
            .ok_or(EntityError::ComponentDataDoesNotExist)?
            .read().unwrap();
        Ok(borrowed_component)
    }

    pub fn get_component_mut<T: Any>(&self) -> Result<RwLockWriteGuard<dyn Any + Send + Sync>, EntityError> {
        let components = self.extract_components::<T>()?;
        let borrowed_component = components[self.id]
            .as_ref()
            .ok_or(EntityError::ComponentDataDoesNotExist)?
            .write().unwrap();
        Ok(borrowed_component)
    }
}
