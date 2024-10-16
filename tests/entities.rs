use std::{
    any::Any,
    sync::{Arc, RwLock},
};

use magma_ecs::World;

#[test]
fn create_entity() {
    let world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    let mut entities = world.entities_write();
    entities
        .create_entity()
        .with_component(Location(32.0, 32.0))
        .unwrap()
        .with_component(Size(10.0))
        .unwrap();
}

#[test]
fn entity_query() {
    let world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    let mut entities = world.entities_write();
    entities
        .create_entity()
        .with_component(Location(32.0, 32.0))
        .unwrap()
        .with_component(Size(10.0))
        .unwrap();
    entities
        .create_entity()
        .with_component(Location(33.0, 33.0))
        .unwrap();
    entities.create_entity().with_component(Size(11.0)).unwrap();

    #[allow(deprecated)]
    let query = entities
        .query()
        .with_component::<Location>()
        .unwrap()
        .with_component::<Size>()
        .unwrap()
        .run();

    #[allow(deprecated)]
    let locations: &Vec<Arc<RwLock<dyn Any + Send + Sync>>> = &query.components[0];
    #[allow(deprecated)]
    let sizes: &Vec<Arc<RwLock<dyn Any + Send + Sync>>> = &query.components[1];

    let borrowed_first_location = locations[0].read().unwrap();
    let first_location = borrowed_first_location.downcast_ref::<Location>().unwrap();

    assert!(locations.len() == sizes.len() && locations.len() == 1 && first_location.0 == 32.0);
}

#[test]
fn delete_component_from_entity() {
    let world = World::new();

    world.register_component::<Location>();
    world.register_component::<Size>();

    let mut entities = world.entities_write();
    entities
        .create_entity()
        .with_component(Location(10.0, 11.0))
        .unwrap()
        .with_component(Size(10.0))
        .unwrap();

    entities
        .create_entity()
        .with_component(Location(20.0, 11.0))
        .unwrap()
        .with_component(Size(20.0))
        .unwrap();

    entities
        .remove_component_by_entity_id::<Location>(0)
        .unwrap();

    #[allow(deprecated)]
    let _query = entities
        .query()
        .with_component::<Location>()
        .unwrap()
        .with_component::<Size>()
        .unwrap()
        .run();
}

#[test]
fn add_component_to_entity() {
    let world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();
    let mut entities = world.entities_write();
    entities
        .create_entity()
        .with_component(Location(10.0, 15.0))
        .unwrap();

    entities.add_component_by_entity_id(Size(20.0), 0).unwrap();

    #[allow(deprecated)]
    let _query = entities
        .query()
        .with_component::<Location>()
        .unwrap()
        .with_component::<Size>()
        .unwrap()
        .run();
}

#[test]
fn delete_entity() {
    let world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    let mut entities = world.entities_write();
    entities
        .create_entity()
        .with_component(Location(10.0, 15.0))
        .unwrap();
    entities
        .create_entity()
        .with_component(Location(20.0, 25.0))
        .unwrap();

    entities.delete_entity_by_id(0).unwrap();

    #[allow(deprecated)]
    let query = entities.query().with_component::<Location>().unwrap().run();

    #[allow(deprecated)]
    let borrowed_location = query.components[0][0].read().unwrap();
    let _location = borrowed_location.downcast_ref::<Location>().unwrap();

    entities
        .create_entity()
        .with_component(Location(30.0, 35.0))
        .unwrap();

    #[allow(deprecated)]
    let query = entities.query().with_component::<Location>().unwrap().run();
    #[allow(deprecated)]
    let borrowed_location = query.components[0][0].read().unwrap();
    let location = borrowed_location.downcast_ref::<Location>().unwrap();

    assert_eq!(location.0, 30.0);
}

#[allow(dead_code)]
struct Location(pub f32, pub f32);
#[allow(dead_code)]
struct Size(pub f32);
