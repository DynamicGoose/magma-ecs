use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::error::EntityError;

use super::Entities;

type ExtractedComponents<'a> = Result<&'a Vec<Option<Rc<RefCell<dyn Any>>>>, EntityError>;

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
    pub fn get_component<T: Any>(&self) -> Result<Ref<T>, EntityError> {
        let components = self.extract_components::<T>()?;
        let borrowed_component = components[self.id]
            .as_ref()
            .ok_or(EntityError::ComponentDataDoesNotExist)?
            .borrow();
        Ok(Ref::map(borrowed_component, |any| {
            any.downcast_ref::<T>().unwrap()
        }))
    }

    pub fn get_component_mut<T: Any>(&self) -> Result<RefMut<T>, EntityError> {
        let components = self.extract_components::<T>()?;
        let borrowed_component = components[self.id]
            .as_ref()
            .ok_or(EntityError::ComponentDataDoesNotExist)?
            .borrow_mut();
        Ok(RefMut::map(borrowed_component, |any| {
            any.downcast_mut::<T>().unwrap()
        }))
    }
}
