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
	world
        .spawn()
        .with_component(Location(33.0, 33.0))
        .unwrap();
	world
        .spawn()
        .with_component(Size(11.0))
        .unwrap();
}

struct Location(pub f32, pub f32);
struct Size(pub f32);
