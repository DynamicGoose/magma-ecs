use std::any::{Any, TypeId};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::error::EntityError;

use super::{query_entity::QueryEntity, Component, Entities};

/// Used for querying for entities with specified components
#[derive(Debug)]
pub struct Query<'a> {
    map: u128,
    entities: &'a Entities,
    type_ids: Vec<TypeId>,
}

/// Result of a [`Query`] with indexes of the found entites and the queried component vecs.
#[deprecated(
    since = "0.1.0",
    note = "This will be removed in 0.2.0 in favor of `QueryEntity`."
)]
pub struct QueryResult {
    pub indexes: Vec<usize>,
    pub components: Vec<Vec<Component>>,
}

impl<'a> Query<'a> {
    pub fn new(entities: &'a Entities) -> Self {
        Self {
            entities,
            map: 0,
            type_ids: vec![],
        }
    }

    /// Add component to the [`Query`]
    pub fn with_component<T: Any>(&mut self) -> Result<&mut Self, EntityError> {
        let type_id = TypeId::of::<T>();
        if let Some(bit_mask) = self.entities.get_bitmask(&type_id) {
            self.map |= bit_mask;
            self.type_ids.push(type_id);
        } else {
            return Err(EntityError::ComponentNotRegistered);
        }
        Ok(self)
    }

    /// Different run method with easier to use API
    pub fn run<R: FnOnce(Vec<QueryEntity>)>(&self, runner: R) {
        runner(
            self.entities
                .map
                .par_iter()
                .enumerate()
                .filter_map(|(index, entity_map)| {
                    if entity_map & self.map == self.map {
                        Some(QueryEntity::new(index, self.entities))
                    } else {
                        None
                    }
                })
                .collect(),
        );
    }
}

#[cfg(test)]
mod test {}
