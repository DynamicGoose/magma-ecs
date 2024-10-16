use std::{
    any::{Any, TypeId},
    sync::{Arc, RwLock},
};

use crate::error::EntityError;

use super::Entities;

type ExtractedComponents<'a> =
    Result<&'a Vec<Option<Arc<RwLock<dyn Any + Send + Sync>>>>, EntityError>;

/// A query entity with the entities id and a reference to the [`Entities`] struct.
pub struct QueryEntity<'a> {
    pub id: usize,
    entities: &'a Entities,
}

impl<'a> QueryEntity<'a> {
    pub fn new(id: usize, entities: &'a Entities) -> Self {
        Self { id, entities }
    }

    fn extract_components<T: Any + Send + Sync>(&self) -> ExtractedComponents {
        let type_id = TypeId::of::<T>();
        self.entities
            .components
            .get(&type_id)
            .ok_or(EntityError::ComponentNotInQuery)
    }

    /// Operate o reference to component
    pub fn component_ref<T: Any + Send + Sync, R: FnOnce(&T)>(
        &self,
        run: R,
    ) -> Result<(), EntityError> {
        let components = self.extract_components::<T>()?;
        let borrowed_component = components[self.id]
            .as_ref()
            .ok_or(EntityError::ComponentDataDoesNotExist)?
            .read()
            .unwrap();
        run(borrowed_component.downcast_ref::<T>().unwrap());
        Ok(())
    }

    /// Operate on specified component of entity. Returns error, when component doesn't exist.
    pub fn component_mut<T: Any + Send + Sync, R: FnOnce(&mut T)>(
        &self,
        run: R,
    ) -> Result<(), EntityError> {
        let components = self.extract_components::<T>()?;
        let mut borrowed_component = components[self.id]
            .as_ref()
            .ok_or(EntityError::ComponentDataDoesNotExist)?
            .write()
            .unwrap();
        run(borrowed_component.downcast_mut::<T>().unwrap());
        Ok(())
    }
}
