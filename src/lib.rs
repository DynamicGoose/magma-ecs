use std::any::Any;

use entities::{query::Query, Entities};
use errors::MecsErrors;
use resources::Resources;

pub mod entities;
pub mod errors;

mod resources;

/// The `World` struct holds all the data of our world.
#[derive(Default)]
pub struct World {
    resources: Resources,
    entities: Entities,
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
    pub fn add_resource(&mut self, resource_data: impl Any) {
        self.resources.add(resource_data);
    }

    /**
    Get an immutable reference to a resource.
    ```
    use magma_ecs::World;

    let mut world = World::new();
    world.add_resource(10_u32);
    let resource = world.get_resource::<u32>().unwrap();
    assert_eq!(*resource, 10);
    ```
    */
    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }

    /**
    Get a mutable reference to a resource. The type of the resource must be added using turbofish notation.
    ```
    use magma_ecs::World;

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

    /// There is currently a limit of 128 components per `World`. This will be improved in the future.
    pub fn register_component<T: Any>(&mut self) {
        self.entities.register_component::<T>();
    }

    /// Spawns a new entity
    pub fn spawn(&mut self) -> &mut Entities {
        self.entities.create_entity()
    }

    /// Query for entities with specified components. Use either `run()` to get a `QueryResult` or `run_entity` to get a `Vec` of `QueryEntity`.
    /// ```
    /// use magma_ecs::World;
    ///
    /// let mut world = World::new();
    /// world.register_component::<u32>();
    /// world.spawn().with_component(32_u32).unwrap();
    ///
    /// let query = world
    ///     .query()
    ///     .with_component::<u32>()
    ///     .unwrap()
    ///     .run();
    ///
    /// let query = world
    ///     .query()
    ///     .with_component::<u32>()
    ///     .unwrap()
    ///     .run_entity();
    /// ```
    pub fn query(&self) -> Query {
        Query::new(&self.entities)
    }

    /// Remove a component from an entity
    pub fn remove_component<T: Any>(&mut self, index: usize) -> Result<(), MecsErrors> {
        self.entities.remove_component_by_entity_id::<T>(index)
    }

    /// Adds the supplied component to the entity at the supplied index
    pub fn add_component(&mut self, data: impl Any, index: usize) -> Result<(), MecsErrors> {
        self.entities.add_component_by_entity_id(data, index)
    }

    /// Despawns the supplied entity
    pub fn despawn(&mut self, index: usize) -> Result<(), MecsErrors> {
        self.entities.delete_entity_by_id(index)
    }

    /**
    This takes a `Vec` of references to functions that take a reference to `World` as well as a `Vec` of references to functions that take a mutable reference to `World`.
    It runs all of the supplied functions once on the `World`.
    */
    pub fn startup(
        &mut self,
        systems_ref: Vec<&dyn Fn(&Self)>,
        systems_mut: Vec<&dyn Fn(&mut Self)>,
    ) {
        for system in systems_ref {
            system(self);
        }
        for system in systems_mut {
            system(self);
        }
    }

    /**
    This takes a `Vec` of references to functions that take a reference to `World` as well as a `Vec` of references to functions that take a mutable reference to `World`.
    It runs all of the supplied functions once on each update.
    It also takes an update condition, which must return `true` for the update loop to run.
    */
    pub fn update(
        &mut self,
        update_condition: &dyn Fn(&Self) -> bool,
        systems_ref: Vec<&dyn Fn(&Self)>,
        systems_mut: Vec<&dyn Fn(&mut Self)>,
    ) {
        while update_condition(self) {
            for system in &systems_ref {
                system(self);
            }
            for system in &systems_mut {
                system(self);
            }
        }
    }
}

#[cfg(test)]
mod tests {}
