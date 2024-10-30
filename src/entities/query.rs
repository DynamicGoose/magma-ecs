use std::any::{Any, TypeId};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use roaring::RoaringBitmap;

use crate::error::EntityError;

use super::{query_entity::QueryEntity, Entities};

/// Used for querying for entities with specified components
#[derive(Debug)]
pub struct Query<'a> {
    map: RoaringBitmap,
    entities: &'a Entities,
    type_ids: Vec<TypeId>,
}

impl<'a> Query<'a> {
    pub(crate) fn new(entities: &'a Entities) -> Self {
        Self {
            entities,
            map: RoaringBitmap::new(),
            type_ids: vec![],
        }
    }

    /// Add component to the [`Query`]
    pub fn with_component<T: Any>(&mut self) -> Result<&mut Self, EntityError> {
        let type_id = TypeId::of::<T>();
        if let Some(bit_mask) = self.entities.get_bitmask(&type_id) {
            self.map.insert(*bit_mask);
            self.type_ids.push(type_id);
        } else {
            return Err(EntityError::ComponentNotRegistered);
        }
        Ok(self)
    }

    /// Run the [`Query`]. This takes a closure to be run on the output, which is a `Vec<[`QueryEntity`]>`.
    /// ```
    /// use magma_ecs::World;
    ///
    /// let mut world = World::new();
    /// world.register_component::<u32>();
    /// world.create_entity((20_u32,)).unwrap();
    ///
    /// world.query()
    ///     .with_component::<u32>()
    ///     .unwrap()
    ///     .run(|entities| {
    ///         // do something with the entities
    ///     });
    /// ```
    pub fn run<R: FnOnce(Vec<QueryEntity>)>(&self, runner: R) {
        let entities = self
            .entities
            .map
            .read()
            .par_iter()
            .enumerate()
            .filter_map(|(index, entity_map)| {
                if self.map.is_subset(entity_map) {
                    Some(QueryEntity::new(index, self.entities))
                } else {
                    None
                }
            })
            .collect();

        runner(entities);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn query_with_component() {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();
        let mut query = Query::new(&entities);
        query
            .with_component::<u32>()
            .unwrap()
            .with_component::<f32>()
            .unwrap();

        assert!(query.map.contains_range(1..2));
        assert_eq!(TypeId::of::<u32>(), query.type_ids[0]);
        assert_eq!(TypeId::of::<f32>(), query.type_ids[1]);
    }

    #[test]
    fn run_query() {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();
        entities.create_entity((10_u32, 20.0_f32)).unwrap();
        entities.create_entity((5_u32,)).unwrap();
        entities.create_entity((20.0_f32,)).unwrap();
        entities.create_entity((15_u32, 25.0_f32)).unwrap();

        Query::new(&entities)
            .with_component::<u32>()
            .unwrap()
            .with_component::<f32>()
            .unwrap()
            .run(|entities| {
                assert_eq!(entities.len(), 2);
                entities[0]
                    .component_ref(|comp: &u32| assert!(*comp == 10 || *comp == 15))
                    .unwrap()
            });
    }
}
