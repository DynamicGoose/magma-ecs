use magma_ecs::World;

#[test]
fn startup() {
    let mut world = World::new();
    world.register_component::<u32>();
    world.update(vec![], vec![&create_u32_entity]);

    let entities = world.entities_read();
    let query = entities.query().with_component::<u32>().unwrap().run();
    assert_eq!(query.indexes.len(), 1);
}

fn create_u32_entity(world: &mut World) {
    let mut entities = world.entities_write();
    entities.create_entity().with_component(10_u32).unwrap();
}
