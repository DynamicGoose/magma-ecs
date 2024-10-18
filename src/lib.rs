//! This crate provides the Entity-Component-System of the [Magma3D-Engine](https://dynamicgoose.github.io/magma3d-engine/).
//!
//! The crate provides a [`World`] struct with [`Resources`] and [`Entities`].
//! An entity is just an index into the component storage.
//! A resource is like a global component, independant from the [`Entities`].
//!

use std::any::Any;

use entities::{query::Query, Entities};
use error::ResourceError;
use resources::Resources;

pub mod entities;
pub mod error;
pub mod resources;
pub mod systems;

/// The [`World`] struct holds all the data of our world.
/// <div class="warning">
///
/// Be careful with acquiring read/write locks. If you try to acquire a lock **while the current function holds another lock**, they will **deadlock**!
///
/// </div>
#[derive(Default)]
pub struct World {
    resources: Resources,
    entities: Entities,
}

impl World {
    /// Creates a new [`World`].
    pub fn new() -> Self {
        Self::default()
    }

    /**
    This adds a resource to the [`World`]'s [`Resources`], which can be of any type that implements the [`Any`], [`Send`] and [`Sync`] traits.
    [`Send`] and [`Sync`] are required for thread safety. **Don't use if you currently hold a lock on the [`Resources`]!**
    */
    pub fn add_resource(&mut self, resource_data: impl Any + Send + Sync) {
        self.resources.add(resource_data);
    }

    /**
    Removes the requested resource from the [`World`]'s [`Resources`] if it exists.
    Use turbofish notation.
     */
    pub fn remove_resource<T: Any + Send + Sync>(&mut self) {
        self.resources.remove::<T>();
    }

    pub fn resource_ref<T: Any + Send + Sync, R: FnOnce(&T)>(
        &self,
        run: R,
    ) -> Result<(), ResourceError> {
        self.resources.resource_ref(run)
    }

    pub fn resource_mut<T: Any + Send + Sync, R: FnOnce(&mut T)>(
        &self,
        run: R,
    ) -> Result<(), ResourceError> {
        self.resources.resource_mut(run)
    }

    /// Register a component.
    /// There is currently a limit of 128 components per `World`. This will be improved in the future.
    pub fn register_component<T: Any + Send + Sync>(&mut self) {
        self.entities.register_component::<T>();
    }

    pub fn create_entity(&self) -> &Entities {
        self.entities.create_entity()
    }

    pub fn query(&self) -> Query {
        self.entities.query()
    }
}

#[cfg(test)]
mod tests {}
