use magma_ecs::World;

#[test]
fn create_entity() {
    let mut world = World::new();
    world.register_component::<u64>();

    world.create_entity().with_component(400_u64).unwrap();
}

#[test]
fn query() {
    let mut world = World::new();
    world.register_component::<u32>();
    world.create_entity().with_component(32_u32).unwrap();

    let mut bool = false;
    world
        .query()
        .with_component::<u32>()
        .unwrap()
        .run(|query_entities| {
            bool = !query_entities.is_empty();
            for entity in query_entities {
                *entity
                    .get_component_mut::<u32>()
                    .unwrap()
                    .downcast_mut::<u32>()
                    .unwrap() += 2;
            }
        });
    assert!(bool);
}
