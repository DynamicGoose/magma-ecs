use std::{any::Any, sync::{RwLock, RwLockReadGuard, RwLockWriteGuard}};

use entities::Entities;
use error::EntityError;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use resources::Resources;

pub mod entities;
pub mod error;

pub mod resources;

/// The `World` struct holds all the data of our world.
#[derive(Default)]
pub struct World {
    resources: RwLock<Resources>,
    entities: RwLock<Entities>,
}

impl World {
    /// Creates a new `World`. Must be `mut` to be operated on.
    pub fn new() -> Self {
        Self::default()
    }

    /**
    This adds a resource, which can be of any type that implements the `std::any::Any` trait.
    ```
    use magma_ecs::World;

    let mut world = World::new();
    world.add_resource(10_u32);
    ```
    */
    pub fn add_resource(&self, resource_data: impl Any + Send + Sync) {
        self.resources.write().unwrap().add(resource_data);
    }

    /**
    Get an immutable reference to a resource. The type of the resource must be added using turbofish notation.
    ```
    use magma_ecs::World;
    use magma_ecs::resources::Resources;

    let mut world = World::new();
    world.add_resource(10_u32);
    // get readlock on resources
    let resources = world.resources_read();
    // get resource
    let resource = resources.get_ref::<u32>().unwrap();
    assert_eq!(*resource, 10);
    ```
    */
    /// Returns a readlock on the world's resources
    pub fn resources_read(&self) -> RwLockReadGuard<Resources> {
        self.resources.read().unwrap()
    }

    /**
    Get a mutable reference to a resource. The type of the resource must be added using turbofish notation.
    ```
    use magma_ecs::World;
    use magma_ecs::resources::Resources;

    let mut world = World::new();
    world.add_resource(10_u32);
    {
        let mut resources = world.resources_write();
        let resource = resources.get_mut::<u32>().unwrap();
        *resource += 1;
    }
    let resources = world.resources_read();
    let resource = resources.get_ref::<u32>().unwrap();
    assert_eq!(*resource, 11);
    ```
    */
    /// Returns a writelock on the world's resources
    pub fn resources_write(&self) -> RwLockWriteGuard<Resources> {
        self.resources.write().unwrap()
    }

    /// Removes the requested resource from the world if it exists.
    pub fn remove_resource<T: Any>(&self) {
        self.resources.write().unwrap().remove::<T>();
    }

    /// There is currently a limit of 128 components per `World`. This will be improved in the future.
    pub fn register_component<T: Any>(&self) {
        self.entities.write().unwrap().register_component::<T>();
    }

    // TODO: Inform about Deadlocks!!!

    /// Returns a readlock on the world's entities
    pub fn entities_read(&self) -> RwLockReadGuard<Entities> {
        self.entities.read().unwrap()
    }

    /// Returns a writelock on the world's entities
    pub fn entities_write(&self) -> RwLockWriteGuard<Entities> {
        self.entities.write().unwrap()
    }

    /// Query for entities with specified components. Use either `run()` to get a `QueryResult` or `run_entity` to get a `Vec` of `QueryEntity`.
    /// ```
    /// use magma_ecs::World;
    ///
    /// let mut world = World::new();
    /// world.register_component::<u32>();
    /// 
    /// let mut entities = world.entities_write();
    /// entities.create_entity().with_component(32_u32).unwrap();
    ///
    /// let query = entities
    ///     .query()
    ///     .with_component::<u32>()
    ///     .unwrap()
    ///     .run();
    ///
    /// let query = entities
    ///     .query()
    ///     .with_component::<u32>()
    ///     .unwrap()
    ///     .run_entity();
    /// ```

    /// Remove a component from an entity
    pub fn remove_component<T: Any>(&self, index: usize) -> Result<(), EntityError> {
        self.entities.write().unwrap().remove_component_by_entity_id::<T>(index)
    }

    /// Adds the supplied component to the entity at the supplied index
    pub fn add_component(&self, data: impl Any + Send + Sync, index: usize) -> Result<(), EntityError> {
        self.entities.write().unwrap().add_component_by_entity_id(data, index)
    }

    /**
    This takes a `Vec` of references to functions that take a reference to `World` as well as a `Vec` of references to functions that take a mutable reference to `World`.
    It runs all of the supplied functions once on the `World`.
    */
    pub fn update(&self, systems: Vec<fn(&Self)>) {
        systems.par_iter().for_each(|s| s(self));
    }
}

#[cfg(test)]
mod tests {}
