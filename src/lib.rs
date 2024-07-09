use std::{
    any::Any,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use entities::Entities;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use resources::Resources;

pub mod entities;
pub mod error;

pub mod resources;

/// The [`World`] struct holds all the data of our world.
/// <div class="warning">
///
/// Be careful with acquiring read/write locks. If you try to acquire a lock **while the current funktion holds another lock**, they will **deadlock**!
///
/// </div>
#[derive(Default)]
pub struct World {
    resources: RwLock<Resources>,
    entities: RwLock<Entities>,
}

impl World {
    /// Creates a new [`World`].
    pub fn new() -> Self {
        Self::default()
    }

    /**
    This adds a resource to the [`World`]'s [`Resources`], which can be of any type that implements the [`Any`], [`Send`] and [`Sync`] traits.
    [`Send`] and [`Sync`] are required for thread safety. **Don't use if you currently hold a lock on the [`Resources`]!**
    ```
    use magma_ecs::World;

    let world = World::new();
    world.add_resource(10_u32);
    ```
    */
    pub fn add_resource(&self, resource_data: impl Any + Send + Sync) {
        self.resources.write().unwrap().add(resource_data);
    }

    /**
    Removes the requested resource from the [`World`]'s [`Resources`] if it exists.
    Use turbofish notation.
    ```
    use magma_ecs::World;

    let world = World::new();
    // add u32 resource
    world.add_resource(10_u32);
    //remove resource
    world.remove_resource::<u32>();

    ```
     */
    pub fn remove_resource<T: Any + Send + Sync>(&self) {
        self.resources.write().unwrap().remove::<T>();
    }

    /**
    Returns a readlock on the [`World`]'s [`Resources`].
    ```
    use magma_ecs::World;

    let world = World::new();

    // acquire readlock on resources
    let resources = world.resources_read();
    // get values from the resources...
    ```
     */
    pub fn resources_read(&self) -> RwLockReadGuard<Resources> {
        self.resources.read().unwrap()
    }

    /**
    Returns a writelock on the [`World`]'s [`Resources`].
    ```
    use magma_ecs::World;

    let world = World::new();

    // acquire readlock on resources
    let mut resources = world.resources_write();
    // modify values in the resources...
    ```
    */
    pub fn resources_write(&self) -> RwLockWriteGuard<Resources> {
        self.resources.write().unwrap()
    }

    /// Register a component.
    /// There is currently a limit of 128 components per `World`. This will be improved in the future.
    pub fn register_component<T: Any + Send + Sync>(&self) {
        self.entities.write().unwrap().register_component::<T>();
    }

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

    /// This takes a [`Vec`] of references to functions that take a reference to [`World`].
    /// It runs all of the supplied functions in parallel once on the [`World`].
    pub fn update(&self, systems: Vec<fn(&Self)>) {
        systems.par_iter().for_each(|s| s(self));
    }
}

#[cfg(test)]
mod tests {}
