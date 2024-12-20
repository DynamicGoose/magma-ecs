//! This crate provides the Entity-Component-System of the [Magma3D-Engine](https://dynamicgoose.github.io/magma3d-engine/).
//!
//! The crate provides a [`World`] struct with [`Resources`] and [`Entities`].
//! An entity is just an index into the component storage.
//! A resource is like a global component, independent of the [`Entities`].
//!
//! Example for creating and setting up a [`World`]:
//! ```
//! use magma_ecs::World;
//!
//! let mut world = World::new();
//!
//! // Register a component type.
//! // This can be any type that implements Any + Send + Sync.
//! world.register_component::<u32>();
//!
//! // add a resource
//! world.add_resource(10_u32);
//!
//! // create an entity with registered component
//! world.create_entity((20_u32,)).unwrap();
//! ```

use std::any::Any;

use entities::{component_set::ComponentSet, query::Query, Entities};
use error::{EntityError, ResourceError};
use resources::Resources;

/// Provides the [`Entities`] struct as well as [`query`](entities::query) and [`query_entity`](entities::query_entity) modules.
pub mod entities;
/// Error types
pub mod error;
/// Provides the [`Resources`] struct.
pub mod resources;
/// Provides the [`Systems`](systems::Systems) struct, from which a [`Dispatcher`](systems::dispatcher::Dispatcher) can be created.
pub mod systems;

/// The [`World`] struct holds all the data of our world.
#[derive(Default, Debug)]
pub struct World {
    resources: Resources,
    entities: Entities,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    /// This adds a resource to the [`World`]'s [`Resources`].
    /// This can be any type that implements the [`Any`], [`Send`] and [`Sync`] traits.
    pub fn add_resource(&self, resource_data: impl Any + Send + Sync) -> Result<(), ResourceError> {
        self.resources.add(resource_data)
    }

    /// Removes the requested resource from the [`World`]'s [`Resources`] if it exists.
    /// Use turbofish notation.
    pub fn remove_resource<T: Any + Send + Sync>(&self) {
        self.resources.remove::<T>();
    }

    /// Calls the provided closure on the a reference to a `resource`.
    /// ```
    /// use magma_ecs::World;
    ///
    /// let world = World::new();
    /// world.add_resource(20_u32);
    ///
    /// // operate on reference to u32 resource
    /// world.resource_ref(|res: &u32| {
    ///     // do something with the &u32
    /// }).unwrap();
    /// ```
    pub fn resource_ref<T: Any + Send + Sync, R: FnOnce(&T)>(
        &self,
        run: R,
    ) -> Result<(), ResourceError> {
        self.resources.resource_ref(run)
    }

    /// Calls the provided closure on the a mutable reference to a `resource`.
    /// ```
    /// use magma_ecs::World;
    ///
    /// let world = World::new();
    /// world.add_resource(20_u32);
    ///
    /// // operate on mutable reference to u32 resource
    /// world.resource_mut(|res: &mut u32| {
    ///     // do something with the &mut u32
    /// }).unwrap();
    /// ```
    pub fn resource_mut<T: Any + Send + Sync, R: FnOnce(&mut T)>(
        &self,
        run: R,
    ) -> Result<(), ResourceError> {
        self.resources.resource_mut(run)
    }

    /// Register a component.
    pub fn register_component<T: Any + Send + Sync>(&mut self) {
        self.entities.register_component::<T>();
    }

    /// Spawn an entity with components. Currently the max size for tuples provided to this method is 10.
    /// ```
    /// use magma_ecs::World;
    ///
    /// let mut world = World::new();
    /// world.register_component::<u32>();
    /// world.register_component::<f32>();
    ///
    /// // spawn entity
    /// world.create_entity((30_u32, 60_f32)).unwrap();
    /// // when only adding one component, put a comma after it for rust to recognise it as a tuple
    /// world.create_entity((20_u32,)).unwrap();
    /// ```
    pub fn create_entity(&self, components: impl ComponentSet) -> Result<(), EntityError> {
        self.entities.create_entity(components)
    }

    /// Spawn a batch of entities with the same components. This is more efficient if you have to spawn large amounts of entities.
    /// ```
    /// use magma_ecs::World;
    ///
    /// let mut world = World::new();
    /// world.register_component::<u32>();
    /// world.register_component::<f32>();
    ///
    /// // spawn 100 entities
    /// world.create_entity_batch((30_u32, 60_f32), 100).unwrap();
    /// ```
    pub fn create_entity_batch(
        &self,
        components: impl ComponentSet,
        num: usize,
    ) -> Result<(), EntityError> {
        self.entities.create_entity_batch(components, num)
    }

    /// Get a [`Query`] on the [`World`]'s [`Entities`].
    pub fn query(&self) -> Query {
        self.entities.query()
    }
}
