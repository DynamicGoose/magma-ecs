use magma_ecs::World;

#[test]
fn startup() {
    let mut world = World::new();
    world.register_component::<u32>();
    world.update(vec![], vec![&create_u32_entity]);
    let query = world.query().with_component::<u32>().unwrap().run();
    assert_eq!(query.indexes.len(), 1);
}

fn create_u32_entity(world: &mut World) {
    world.spawn().with_component(10_u32).unwrap();
}
