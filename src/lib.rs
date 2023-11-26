use std::any::Any;

use entities::{Entities, query::Query};
use resources::Resources;

pub mod errors;

mod entities;
mod internal_errors;
mod resources;

/// The `World` struct holds all the data of our world.
#[derive(Default)]
pub struct World {
    resources: Resources,
    entities: Entities,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    /// This adds a resource, which can be of any type that implements the `std::any::Any` trait.
    pub fn add_resource(&mut self, resource_data: impl Any) {
        self.resources.add(resource_data);
    }

    /// Get an immutable reference to a resource.
    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }

    /**
    Get a mutable reference to a resource. The type of the resource must be added using turbofish notation.
    ```
    use mecs::World;

    let mut world = World::new();
    world.add_resource(10_u32);
    {
        let resource = world.get_resource_mut::<u32>().unwrap();
        *resource += 1;
    }
    let resource = world.get_resource::<u32>().unwrap();
    assert_eq!(*resource, 11);
    ```
    */
    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    /// Removes the requested resource from the world if it exists.
    pub fn remove_resource<T: Any>(&mut self) {
        self.resources.remove::<T>();
    }

    // TODO: combine register_component and with_component
    pub fn register_component<T: Any>(&mut self) {
        self.entities.register_component::<T>();
    }

    pub fn spawn(&mut self) -> &mut Entities {
        self.entities.add_entity()
    }

    pub fn query(&self) -> Query {
        Query::new()
    }
}

#[cfg(test)]
mod tests {}
