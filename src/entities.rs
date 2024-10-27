pub mod query;
/// Output of running a [`Query`]
pub mod query_entity;

use parking_lot::RwLock;

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use query::Query;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use roaring::RoaringBitmap;

use crate::error::EntityError;

pub(crate) type Component = Arc<RwLock<dyn Any + Send + Sync>>;
pub(crate) type ComponentMap = HashMap<TypeId, RwLock<Vec<Option<Component>>>>;

#[derive(Debug, Default)]
pub struct Entities {
    components: ComponentMap,
    bit_masks: HashMap<TypeId, RoaringBitmap>,
    map: RwLock<Vec<RoaringBitmap>>,
    into_index: RwLock<usize>,
}

impl Entities {
    pub(crate) fn register_component<T: Any + Send + Sync>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, RwLock::new(vec![]));
        let mut bit_mask = RoaringBitmap::new();
        bit_mask.insert(1 << self.bit_masks.len());
        self.bit_masks.insert(type_id, bit_mask);
    }

    pub(crate) fn create_entity(&self) -> &Self {
        {
            let mut map = self.map.write();
            if let Some((index, _)) = map
                .par_iter()
                .enumerate()
                .find_any(|(_, mask)| mask.is_empty())
            {
                *self.into_index.write() = index;
            } else {
                self.components
                    .par_iter()
                    .for_each(|(_key, components)| components.write().push(None));
                map.push(RoaringBitmap::new());
                *self.into_index.write() = map.len() - 1;
            }
        }

        self
    }

    /// Add component to an entity on creation. The component has to be registered first.
    pub fn with_component(&self, data: impl Any + Send + Sync) -> Result<&Self, EntityError> {
        let type_id = data.type_id();
        let index = self.into_index.read();
        if let Some(components) = self.components.get(&type_id) {
            let mut components = components.write();
            let component = components
                .get_mut(*index)
                .ok_or(EntityError::ComponentNotRegistered)?;
            *component = Some(Arc::new(RwLock::new(data)));

            let bit_mask = self.bit_masks.get(&type_id).unwrap();
            self.map.write()[*index] |= bit_mask;
        } else {
            return Err(EntityError::ComponentNotRegistered);
        }

        Ok(self)
    }

    pub(crate) fn get_bitmask(&self, type_id: &TypeId) -> Option<&RoaringBitmap> {
        self.bit_masks.get(type_id)
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

        let mut map = self.map.write();
        if &map[index] & mask == *mask {
            map[index] ^= mask;
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
        self.map.write()[index] |= mask;

        let components = self.components.get(&type_id).unwrap();
        components.write()[index] = Some(Arc::new(RwLock::new(data)));
        Ok(())
    }

    pub(crate) fn delete_entity_by_id(&self, index: usize) -> Result<(), EntityError> {
        if let Some(map) = self.map.write().get_mut(index) {
            map.clear();
        } else {
            return Err(EntityError::EntityDoesNotExist);
        }
        Ok(())
    }

    pub(crate) fn query(&self) -> Query {
        Query::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn register_component() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        let type_id = TypeId::of::<Health>();
        let health_components = entities.components.get(&type_id).unwrap();
        assert_eq!(health_components.read().len(), 0);
    }

    #[test]
    fn update_component_masks() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        let type_id = TypeId::of::<Speed>();
        let mask = entities.bit_masks.get(&type_id).unwrap();
        assert!(mask.contains(2));
    }

    #[test]
    fn create_entity() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities.create_entity();

        let health = entities.components.get(&TypeId::of::<Health>()).unwrap();
        let speed = entities.components.get(&TypeId::of::<Speed>()).unwrap();

        assert_eq!(health.read().len(), 1);
        assert_eq!(speed.read().len(), 1);
        assert!(health.read()[0].is_none());
        assert!(speed.read()[0].is_none());
    }

    #[test]
    fn entity_with_component() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities
            .create_entity()
            .with_component(Health(100))
            .unwrap()
            .with_component(Speed(15))
            .unwrap();

        let health = &entities
            .components
            .get(&TypeId::of::<Health>())
            .unwrap()
            .read()[0];
        let health_borrowed = health.as_ref().unwrap().read();
        let health_downcast = health_borrowed.downcast_ref::<Health>().unwrap();

        let speed = &entities
            .components
            .get(&TypeId::of::<Speed>())
            .unwrap()
            .read()[0];
        let speed_borrowed = speed.as_ref().unwrap().read();
        let speed_downcast = speed_borrowed.downcast_ref::<Speed>().unwrap();
        assert_eq!(health_downcast.0, 100);
        assert_eq!(speed_downcast.0, 15);
    }

    #[test]
    fn update_entity_map() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities
            .create_entity()
            .with_component(Health(100))
            .unwrap()
            .with_component(Speed(15))
            .unwrap();
        entities
            .create_entity()
            .with_component(Health(100))
            .unwrap();

        let entity_map = entities.map.read();
        assert!(entity_map[0].contains_range(1..2));
        assert!(entity_map[1].contains(1));
    }

    #[test]
    fn remove_component() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();

        entities
            .create_entity()
            .with_component(Health(10))
            .unwrap()
            .with_component(Speed(50))
            .unwrap();

        entities.remove_component_by_entity_id::<Health>(0).unwrap();

        assert_eq!(entities.map.read()[0].min().unwrap(), 2);
    }

    #[test]
    fn add_component() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities
            .create_entity()
            .with_component(Health(100))
            .unwrap();

        entities.add_component_by_entity_id(Speed(50), 0).unwrap();

        assert!(entities.map.read()[0].contains_range(1..2));
    }

    #[test]
    fn delete_entity_by_id() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities
            .create_entity()
            .with_component(Health(100))
            .unwrap();
        entities.delete_entity_by_id(0).unwrap();

        assert!(entities.map.read()[0].is_empty());
    }

    #[test]
    fn reuse_deleted_entity_columns() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities
            .create_entity()
            .with_component(Health(100))
            .unwrap();
        entities.create_entity().with_component(Health(50)).unwrap();
        entities.delete_entity_by_id(0).unwrap();
        entities.create_entity().with_component(Health(25)).unwrap();

        let health = &entities
            .components
            .get(&TypeId::of::<Health>())
            .unwrap()
            .read()[0];
        let health_borrowed = health.as_ref().unwrap().read();
        let health_downcast = health_borrowed.downcast_ref::<Health>().unwrap();
        assert!(entities.map.read()[0].contains(1));
        assert_eq!(health_downcast.0, 25);
    }

    struct Health(u32);
    struct Speed(u32);
}
