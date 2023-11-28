use std::{any::Any, cell::RefCell, rc::Rc};

use mecs::World;

#[test]
fn create_entity() {
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    world
        .spawn()
        .with_component(Location(32.0, 32.0))
        .unwrap()
        .with_component(Size(10.0))
        .unwrap();
}

#[test]
fn entity_query() {
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    world
        .spawn()
        .with_component(Location(32.0, 32.0))
        .unwrap()
        .with_component(Size(10.0))
        .unwrap();
    world.spawn().with_component(Location(33.0, 33.0)).unwrap();
    world.spawn().with_component(Size(11.0)).unwrap();

    let query = world
        .query()
        .with_component::<Location>()
        .unwrap()
        .with_component::<Size>()
        .unwrap()
        .run();

    let locations: &Vec<Rc<RefCell<dyn Any>>> = &query.components[0];
    let sizes: &Vec<Rc<RefCell<dyn Any>>> = &query.components[1];

    let borrowed_first_location = locations[0].borrow();
    let first_location = borrowed_first_location.downcast_ref::<Location>().unwrap();

    assert!(locations.len() == sizes.len() && locations.len() == 1 && first_location.0 == 32.0);
}

#[test]
fn delete_component_from_entity() {
    let mut world = World::new();

    world.register_component::<Location>();
    world.register_component::<Size>();

    world
        .spawn()
        .with_component(Location(10.0, 11.0))
        .unwrap()
        .with_component(Size(10.0))
        .unwrap();

    world
        .spawn()
        .with_component(Location(20.0, 11.0))
        .unwrap()
        .with_component(Size(20.0))
        .unwrap();

    world.remove_component::<Location>(0).unwrap();

    let query = world
        .query()
        .with_component::<Location>()
        .unwrap()
        .with_component::<Size>()
        .unwrap()
        .run();

    assert!(query.indexes.len() == 1 && query.indexes[0] == 1);
}

#[test]
fn add_component_to_entity() {
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();
    world.spawn().with_component(Location(10.0, 15.0)).unwrap();

    world.add_component(Size(20.0), 0).unwrap();

    let query = world
        .query()
        .with_component::<Location>()
        .unwrap()
        .with_component::<Size>()
        .unwrap()
        .run();

    assert_eq!(query.indexes.len(), 1);
}

#[test]
fn delete_entity() {
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();
    world.spawn().with_component(Location(10.0, 15.0)).unwrap();
    world.spawn().with_component(Location(20.0, 25.0)).unwrap();

    world.despawn(0).unwrap();

    let query = world.query().with_component::<Location>().unwrap().run();

    let borrowed_location = query.components[0][0].borrow();
    let location = borrowed_location.downcast_ref::<Location>().unwrap();

    assert!(query.indexes.len() == 1 && location.0 == 20.0);

    // TODO: implemet reusing deleted entities
    world.spawn().with_component(Location(30.0, 35.0)).unwrap();
    let query = world.query().with_component::<Location>().unwrap().run();
    let borrowed_location = query.components[0][0].borrow();
    let location = borrowed_location.downcast_ref::<Location>().unwrap();

    assert_eq!(location.0, 30.0);
}

struct Location(pub f32, pub f32);
struct Size(pub f32);
