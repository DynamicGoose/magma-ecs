//! This crate provides the Entity-Component-System of the [Magma3D-Engine](https://dynamicgoose.github.io/magma3d-engine/).
//!
//! The crate provides a [`World`] struct with [`Resources`] and [`Entities`].
//! An entity is just an index into the component storage.
//! A resource is like a global component, independant from the [`Entities`].
//!
//! Example for creating and setting up a [`World`]:

use std::{
    any::{Any, TypeId},
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use entities::{query::Query, Entities};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use resources::Resources;

pub mod entities;
pub mod error;

pub mod resources;

/// The [`World`] struct holds all the data of our world.
/// <div class="warning">
///
/// Be careful with acquiring read/write locks. If you try to acquire a lock **while the current function holds another lock**, they will **deadlock**!
///
/// </div>
#[derive(Default)]
pub struct World {
    resources: RwLock<Resources>,
    entities: RwLock<Entities>,
    mod_data: RwLock<ModData>,
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
    pub fn add_resource(&self, resource_data: impl Any + Send + Sync) {
        self.resources.write().unwrap().add(resource_data);
    }

    /**
    Removes the requested resource from the [`World`]'s [`Resources`] if it exists.
    Use turbofish notation.
     */
    pub fn remove_resource<T: Any + Send + Sync>(&self) {
        self.resources.write().unwrap().remove::<T>();
    }

    /**
    Returns a readlock on the [`World`]'s [`Resources`].
     */
    pub fn resources_read(&self) -> RwLockReadGuard<Resources> {
        self.resources.read().unwrap()
    }

    /**
    Returns a writelock on the [`World`]'s [`Resources`].
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

    pub fn create_entity(&self) -> RwLockWriteGuard<Entities> {
        self.entities.write().unwrap().create_entity();
        self.entities.write().unwrap()
    }

    /// This takes a [`Vec`] of references to functions that take a reference to [`World`].
    /// It runs all of the supplied functions in parallel once on the [`World`].
    pub fn update(&self, systems: &Vec<fn(&Self)>) {
        systems.par_iter().for_each(|s| s(self));
    }
}

#[derive(Default)]
struct ModData {
    add_components: Vec<(Box<dyn Any + Send + Sync>, usize)>,
    remove_components: Vec<(TypeId, usize)>,
    delete_entities: Vec<usize>,
}

#[cfg(test)]
mod tests {}
