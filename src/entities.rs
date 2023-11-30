pub mod query;
pub mod query_entity;

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use crate::errors::MecsErrors;

pub type Component = Rc<RefCell<dyn Any>>;
pub type ComponentMap = HashMap<TypeId, Vec<Option<Component>>>;

#[derive(Debug, Default)]
pub struct Entities {
    components: ComponentMap,
    // TODO (0.2.0): Increase bitmask size (bitmaps crate?)
    bit_masks: HashMap<TypeId, u128>,
    map: Vec<u128>,
    into_index: usize,
}

impl Entities {
    pub fn register_component<T: Any>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, vec![]);
        self.bit_masks.insert(type_id, 1 << self.bit_masks.len());
    }

    pub fn create_entity(&mut self) -> &mut Self {
        if let Some((index, _)) = self.map.iter().enumerate().find(|(_, mask)| **mask == 0) {
            self.into_index = index;
        } else {
            self.components
                .iter_mut()
                .for_each(|(_key, components)| components.push(None));
            self.map.push(0);
            self.into_index = self.map.len() - 1;
        }

        self
    }

    /**
    Add component to an entity on creation. The component has to be registered first for this to work.
    ```
    use mecs::World;

    let mut world = World::new();
    world.register_component::<u32>();
    world.spawn().with_component(32_u32).unwrap();
    ```
    */
    pub fn with_component(&mut self, data: impl Any) -> Result<&mut Self, MecsErrors> {
        let type_id = data.type_id();
        let index = self.into_index;
        if let Some(components) = self.components.get_mut(&type_id) {
            let component = components
                .get_mut(index)
                .ok_or(MecsErrors::ComponentNotRegistered)?;
            *component = Some(Rc::new(RefCell::new(data)));

            let bit_mask = self.bit_masks.get(&type_id).unwrap();
            self.map[index] |= *bit_mask;
        } else {
            return Err(MecsErrors::ComponentNotRegistered);
        }
        Ok(self)
    }

    pub fn get_bitmask(&self, type_id: &TypeId) -> Option<u128> {
        self.bit_masks.get(type_id).copied()
    }

    pub fn remove_component_by_entity_id<T: Any>(
        &mut self,
        index: usize,
    ) -> Result<(), MecsErrors> {
        let type_id = TypeId::of::<T>();
        let mask = if let Some(mask) = self.bit_masks.get(&type_id) {
            mask
        } else {
            return Err(MecsErrors::ComponentNotRegistered);
        };

        if self.map[index] & *mask == *mask {
            self.map[index] ^= *mask;
        }
        Ok(())
    }

    pub fn add_component_by_entity_id(
        &mut self,
        data: impl Any,
        index: usize,
    ) -> Result<(), MecsErrors> {
        let type_id = data.type_id();
        let mask = if let Some(mask) = self.bit_masks.get(&type_id) {
            mask
        } else {
            return Err(MecsErrors::ComponentNotRegistered);
        };
        self.map[index] |= *mask;

        let components = self.components.get_mut(&type_id).unwrap();
        components[index] = Some(Rc::new(RefCell::new(data)));
        Ok(())
    }

    pub fn delete_entity_by_id(&mut self, index: usize) -> Result<(), MecsErrors> {
        if let Some(map) = self.map.get_mut(index) {
            *map = 0;
        } else {
            return Err(MecsErrors::EntityDoesNotExist);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::any::TypeId;

    use super::*;

    #[test]
    fn register_component() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        let type_id = TypeId::of::<Health>();
        let health_components = entities.components.get(&type_id).unwrap();
        assert_eq!(health_components.len(), 0);
    }

    #[test]
    fn update_component_masks() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        let type_id = TypeId::of::<Speed>();
        let mask = entities.bit_masks.get(&type_id).unwrap();
        assert_eq!(*mask, 2);
    }

    #[test]
    fn create_entity() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities.create_entity();

        let health = entities.components.get(&TypeId::of::<Health>()).unwrap();
        let speed = entities.components.get(&TypeId::of::<Speed>()).unwrap();

        assert!(
            health.len() == speed.len()
                && health.len() == 1
                && health[0].is_none()
                && speed[0].is_none()
        );
    }

    #[test]
    fn with_component() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities
            .create_entity()
            .with_component(Health(100))
            .unwrap()
            .with_component(Speed(15))
            .unwrap();

        let first_health = &entities.components.get(&TypeId::of::<Health>()).unwrap()[0];
        let wrapped_health = first_health.as_ref().unwrap();
        let borrowed_health = wrapped_health.borrow();
        let health = borrowed_health.downcast_ref::<Health>().unwrap();

        assert_eq!(health.0, 100);
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

        let entity_map = entities.map[1];
        assert_eq!(entity_map, 1);
    }

    #[test]
    fn remove_component_by_entity_id() {
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

        assert_eq!(entities.map[0], 2);
    }

    #[test]
    fn add_component_by_entity_id() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities
            .create_entity()
            .with_component(Health(100))
            .unwrap();

        entities.add_component_by_entity_id(Speed(50), 0).unwrap();

        assert_eq!(entities.map[0], 3);

        let speed_type_id = TypeId::of::<Speed>();
        let wrapped_speeds = entities.components.get(&speed_type_id).unwrap();
        let wrapped_speed = wrapped_speeds[0].as_ref().unwrap();
        let borrowed_speed = wrapped_speed.borrow();
        let speed = borrowed_speed.downcast_ref::<Speed>().unwrap();

        assert!(entities.map[0] == 3 && speed.0 == 50);
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

        assert_eq!(entities.map[0], 0);
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

        let type_id = TypeId::of::<Health>();
        let borrowed_health = entities.components.get(&type_id).unwrap()[0]
            .as_ref()
            .unwrap()
            .borrow();
        let health = borrowed_health.downcast_ref::<Health>().unwrap();

        assert!(entities.map[0] == 1 && health.0 == 25);
    }

    struct Health(u32);
    struct Speed(pub u32);
}
