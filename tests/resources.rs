use magma_ecs::World;

#[test]
fn add_resource() {
    let world = World::new();
    world.add_resource(10_u32).unwrap();
}

#[test]
fn remove_resource() {
    let world = World::new();
    world.add_resource(10_u32).unwrap();
    world.remove_resource::<u32>();
}

#[test]
fn get_resource() {
    let world = World::new();
    world.add_resource(32_u32).unwrap();
    world.resource_mut(|n: &mut u32| *n += 1).unwrap();
    world.resource_ref(|n: &u32| assert_eq!(*n, 33)).unwrap();
}
