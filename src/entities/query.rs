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

    /// Run the [`Query`]
    #[deprecated(
        since = "0.1.0",
        note = "This method will be removed in 0.2.0. Please use `run_entity` instead."
    )]
    #[allow(deprecated)]
    pub fn run(&self) -> QueryResult {
        let indexes: Vec<usize> = self
            .entities
            .map
            .par_iter()
            .enumerate()
            .filter_map(|(index, entity_map)| {
                if entity_map & self.map == self.map {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        let mut components = vec![];

        for type_id in &self.type_ids {
            let entity_components = self.entities.components.get(type_id).unwrap();
            let mut components_to_keep = vec![];
            for index in &indexes {
                components_to_keep.push(entity_components[*index].clone().unwrap());
            }
            components.push(components_to_keep)
        }

        #[allow(deprecated)]
        QueryResult {
            indexes,
            components,
        }
    }

    /// Different run method with easier to use API
    pub fn run_entity(&self) -> Vec<QueryEntity> {
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
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::entities::query_entity::QueryEntity;

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

        assert!(
            query.map == 3
                && TypeId::of::<u32>() == query.type_ids[0]
                && TypeId::of::<f32>() == query.type_ids[1]
        );
    }

    #[test]
    fn run_query() {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();
        entities
            .create_entity()
            .with_component(10_u32)
            .unwrap()
            .with_component(20.0_f32)
            .unwrap();
        entities.create_entity().with_component(5_u32).unwrap();
        entities.create_entity().with_component(20.0_f32).unwrap();
        entities
            .create_entity()
            .with_component(15_u32)
            .unwrap()
            .with_component(25.0_f32)
            .unwrap();
        let mut query = Query::new(&entities);
        query
            .with_component::<u32>()
            .unwrap()
            .with_component::<f32>()
            .unwrap();
        #[allow(deprecated)]
        let query_result = query.run();
        #[allow(deprecated)]
        let u32s = &query_result.components[0];
        dbg!(u32s);
        #[allow(deprecated)]
        let f32s = &query_result.components[1];
        dbg!(f32s);
        #[allow(deprecated)]
        let indexes = &query_result.indexes;
        dbg!(indexes);

        let first_u32 = *u32s[0].read().unwrap().downcast_ref::<u32>().unwrap();
        dbg!(first_u32);

        assert!(
            u32s.len() == f32s.len()
                && u32s.len() == 2
                && first_u32 == 10
                && u32s.len() == indexes.len()
                && indexes[0] == 0
                && indexes[1] == 3
        );
    }

    #[test]
    fn query_for_entity_ref() {
        let mut entities = Entities::default();

        entities.register_component::<u32>();
        entities.register_component::<f32>();
        entities.create_entity().with_component(100_u32).unwrap();
        entities.create_entity().with_component(10.0_f32).unwrap();

        let mut query = Query::new(&entities);
        let entities: Vec<QueryEntity> = query.with_component::<u32>().unwrap().run_entity();

        assert_eq!(entities.len(), 1);
        for entity in entities {
            assert_eq!(entity.id, 0);
            let health_lock = entity.get_component::<u32>().unwrap();
            let health = health_lock.downcast_ref::<u32>().unwrap();
            assert_eq!(*health, 100);
        }
    }

    #[test]
    fn query_for_entity_mut() {
        let mut entities = Entities::default();

        entities.register_component::<u32>();
        entities.register_component::<f32>();
        entities.create_entity().with_component(100_u32).unwrap();
        entities.create_entity().with_component(10.0_f32).unwrap();

        let mut query = Query::new(&entities);
        let entities: Vec<QueryEntity> = query.with_component::<u32>().unwrap().run_entity();

        assert_eq!(entities.len(), 1);
        for entity in entities {
            assert_eq!(entity.id, 0);
            let mut health_lock = entity.get_component_mut::<u32>().unwrap();
            let health = health_lock.downcast_mut::<u32>().unwrap();
            assert_eq!(*health, 100);
            *health += 1;
        }

        let entities: Vec<QueryEntity> = query.with_component::<u32>().unwrap().run_entity();

        for entity in entities {
            let health_lock = entity.get_component::<u32>().unwrap();
            let health = health_lock.downcast_ref::<u32>().unwrap();
            assert_eq!(*health, 101);
        }
    }
}
