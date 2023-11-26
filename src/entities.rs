pub mod query;

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use crate::internal_errors::EntityErrors;

// TODO: Implement better API
#[derive(Debug, Default)]
pub struct Entities {
    // TODO: type definition
    components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
    // this is limited to 32 components
    // TODO: Increase bitmask size
    bit_masks: HashMap<TypeId, u32>,
    map: Vec<u32>,
}

impl Entities {
    pub fn register_component<T: Any>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, vec![]);
        self.bit_masks
            .insert(type_id, 2_u32.pow(self.bit_masks.len() as u32));
    }

    pub fn create_entity(&mut self) -> &mut Self {
        self.components
            .iter_mut()
            .for_each(|(_key, components)| components.push(None));
        self.map.push(0);
        self
    }

    pub fn with_component(&mut self, data: impl Any) -> Result<&mut Self, EntityErrors> {
        let type_id = data.type_id();
        let map_index = self.map.len() - 1;
        if let Some(components) = self.components.get_mut(&type_id) {
            let last_component = components
                .last_mut()
                .ok_or(EntityErrors::ComponentNotRegistered)?;
            *last_component = Some(Rc::new(RefCell::new(data)));

            let bit_mask = self.bit_masks.get(&type_id).unwrap();
            self.map[map_index] |= *bit_mask;
        } else {
            return Err(EntityErrors::ComponentNotRegistered);
        }
        Ok(self)
    }

    pub fn get_bitmask(&self, type_id: &TypeId) -> Option<u32> {
        self.bit_masks.get(type_id).copied()
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
    fn add_entity() {
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

    struct Health(u32);
    struct Speed(pub u32);
}
