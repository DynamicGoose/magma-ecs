pub mod query;

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use crate::errors::MecsErrors;

type ComponentMap = HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>;

// TODO: Implement better API
#[derive(Debug, Default)]
pub struct Entities {
    components: ComponentMap,
    // TODO (0.2.0): Increase bitmask size (bitmaps crate?)
    bit_masks: HashMap<TypeId, u128>,
    map: Vec<u128>,
}

impl Entities {
    pub fn register_component<T: Any>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, vec![]);
        self.bit_masks
            .insert(type_id, 2_u128.pow(self.bit_masks.len() as u32));
    }

    pub fn create_entity(&mut self) -> &mut Self {
        self.components
            .iter_mut()
            .for_each(|(_key, components)| components.push(None));
        self.map.push(0);
        self
    }

    pub fn with_component(&mut self, data: impl Any) -> Result<&mut Self, MecsErrors> {
        let type_id = data.type_id();
        let map_index = self.map.len() - 1;
        if let Some(components) = self.components.get_mut(&type_id) {
            let last_component = components
                .last_mut()
                .ok_or(MecsErrors::ComponentNotRegistered)?;
            *last_component = Some(Rc::new(RefCell::new(data)));

            let bit_mask = self.bit_masks.get(&type_id).unwrap();
            self.map[map_index] |= *bit_mask;
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

        self.map[index] ^= *mask;
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

    struct Health(u32);
    struct Speed(pub u32);
}
