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
    components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
}

impl Entities {
    pub fn register_component<T: Any>(&mut self) {
        self.components.insert(TypeId::of::<T>(), vec![]);
    }

    pub fn new_entity(&mut self) -> &mut Self {
        self.components
            .iter_mut()
            .for_each(|(_key, components)| components.push(None));
        self
    }

    pub fn with_component(&mut self, data: impl Any) -> Result<&mut Self, EntityErrors> {
        let type_id = data.type_id();
        if let Some(components) = self.components.get_mut(&type_id) {
            // TODO: implement error handling
            let last_component = components
                .last_mut()
                .ok_or(EntityErrors::ComponentNeverRegistered)?;
            *last_component = Some(Rc::new(RefCell::new(data)))
        } else {
            return Err(EntityErrors::ComponentNotRegistered.into());
        }
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use std::any::TypeId;

    use super::*;

    #[test]
    fn register_entity() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        let type_id = TypeId::of::<Health>();
        let health_components = entities.components.get(&type_id).unwrap();
        assert_eq!(health_components.len(), 0);
    }

    #[test]
    fn create_entity() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities.new_entity();

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
            .new_entity()
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

    struct Health(u32);
    struct Speed(pub u32);
}
