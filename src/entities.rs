//! Provides the [`Entities`] struct as well as [`query`] and [`query_entity`] modules.

/// Basic query functionality
pub mod query;
/// Easier to use query API
pub mod query_entity;

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, RwLock},
};

use query::Query;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::error::EntityError;

pub(crate) type Component = Arc<RwLock<dyn Any + Send + Sync>>;
pub(crate) type ComponentMap = HashMap<TypeId, RwLock<Vec<Option<Component>>>>;

#[derive(Debug, Default)]
pub struct Entities {
    components: ComponentMap,
    // TODO (0.2.0): Increase bitmask size (bitmaps crate?)
    bit_masks: HashMap<TypeId, u128>,
    map: RwLock<Vec<u128>>,
    into_index: RwLock<usize>,
}

impl Entities {
    pub(crate) fn register_component<T: Any + Send + Sync>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, RwLock::new(vec![]));
        self.bit_masks.insert(type_id, 1 << self.bit_masks.len());
    }

    /// Create an entity.
    pub(crate) fn create_entity(&self) -> &Self {
        {
            let mut map = self.map.write().unwrap();
            if let Some((index, _)) = map.par_iter().enumerate().find_any(|(_, mask)| **mask == 0) {
                *self.into_index.write().unwrap() = index;
            } else {
                self.components
                    .par_iter()
                    .for_each(|(_key, components)| components.write().unwrap().push(None));
                map.push(0);
                *self.into_index.write().unwrap() = map.len() - 1;
            }
        }

        self
    }

    /**
    Add component to an entity on creation. The component has to be registered first for this to work.
    */
    pub fn with_component(&self, data: impl Any + Send + Sync) -> Result<&Self, EntityError> {
        let type_id = data.type_id();
        {
            let index = self.into_index.read().unwrap();
            if let Some(components) = self.components.get(&type_id) {
                let mut components = components.write().unwrap();
                let component = components
                    .get_mut(*index)
                    .ok_or(EntityError::ComponentNotRegistered)?;
                *component = Some(Arc::new(RwLock::new(data)));

                let bit_mask = self.bit_masks.get(&type_id).unwrap();
                self.map.write().unwrap()[*index] |= *bit_mask;
            } else {
                return Err(EntityError::ComponentNotRegistered);
            }
        }

        Ok(self)
    }

    pub(crate) fn get_bitmask(&self, type_id: &TypeId) -> Option<u128> {
        self.bit_masks.get(type_id).copied()
    }

    pub(crate) fn remove_component_by_entity_id<T: Any + Send + Sync>(
        &self,
        index: usize,
    ) -> Result<(), EntityError> {
        let type_id = TypeId::of::<T>();
        let mask = if let Some(mask) = self.bit_masks.get(&type_id) {
            mask
        } else {
            return Err(EntityError::ComponentNotRegistered);
        };

        let mut map = self.map.write().unwrap();
        if map[index] & *mask == *mask {
            map[index] ^= *mask;
        }
        Ok(())
    }

    pub(crate) fn add_component_by_entity_id(
        &self,
        data: impl Any + Send + Sync,
        index: usize,
    ) -> Result<(), EntityError> {
        let type_id = data.type_id();
        let mask = if let Some(mask) = self.bit_masks.get(&type_id) {
            mask
        } else {
            return Err(EntityError::ComponentNotRegistered);
        };
        self.map.write().unwrap()[index] |= *mask;

        let components = self.components.get(&type_id).unwrap();
        components.write().unwrap()[index] = Some(Arc::new(RwLock::new(data)));
        Ok(())
    }

    pub(crate) fn delete_entity_by_id(&self, index: usize) -> Result<(), EntityError> {
        if let Some(map) = self.map.write().unwrap().get_mut(index) {
            *map = 0;
        } else {
            return Err(EntityError::EntityDoesNotExist);
        }
        Ok(())
    }

    /// Query for entities with specified components. Use either `run()` to get a `QueryResult` or `run_entity` to get a `Vec` of `QueryEntity`.
    pub fn query(&self) -> Query {
        Query::new(self)
    }
}

#[cfg(test)]
mod test {}
