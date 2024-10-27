use parking_lot::{RwLock, RwLockReadGuard};
use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use crate::error::EntityError;

use super::Entities;

type ExtractedComponents<'a> =
    Result<RwLockReadGuard<'a, Vec<Option<Arc<RwLock<dyn Any + Send + Sync>>>>>, EntityError>;

/// A query entity with the entities id and a reference to the [`Entities`] struct.
pub struct QueryEntity<'a> {
    pub id: usize,
    entities: &'a Entities,
}

impl<'a> QueryEntity<'a> {
    pub(crate) fn new(id: usize, entities: &'a Entities) -> Self {
        Self { id, entities }
    }

    fn extract_components<T: Any + Send + Sync>(&self) -> ExtractedComponents {
        let type_id = TypeId::of::<T>();
        Ok(self
            .entities
            .components
            .get(&type_id)
            .ok_or(EntityError::ComponentNotInQuery)?
            .read())
    }

    /// Operate on reference to component. Returns an error if the component doesn't exist.
    pub fn component_ref<T: Any + Send + Sync, R: FnOnce(&T)>(
        &self,
        run: R,
    ) -> Result<(), EntityError> {
        let components = self.extract_components::<T>()?;
        let borrowed_component = components[self.id]
            .as_ref()
            .ok_or(EntityError::ComponentDataDoesNotExist)?
            .read();
        run(borrowed_component.downcast_ref::<T>().unwrap());
        Ok(())
    }

    /// Operate on mutable reference to component. Returns an error if component doesn't exist.
    pub fn component_mut<T: Any + Send + Sync, R: FnOnce(&mut T)>(
        &self,
        run: R,
    ) -> Result<(), EntityError> {
        let components = self.extract_components::<T>()?;
        let mut borrowed_component = components[self.id]
            .as_ref()
            .ok_or(EntityError::ComponentDataDoesNotExist)?
            .write();
        run(borrowed_component.downcast_mut::<T>().unwrap());
        Ok(())
    }

    /// Remove specified component from entity
    pub fn remove_component<T: Any + Send + Sync>(&self) -> Result<(), EntityError> {
        self.entities.remove_component_by_entity_id::<T>(self.id)
    }

    /// Add component to entity
    pub fn add_component(&self, data: impl Any + Send + Sync) -> Result<(), EntityError> {
        self.entities.add_component_by_entity_id(data, self.id)
    }

    /// Delete this entity
    pub fn delete(self) {
        self.entities.delete_entity_by_id(self.id).unwrap();
    }
}
