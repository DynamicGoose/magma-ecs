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
                let mut _test = 2;
                entity
                    .component_mut(|data: &mut u32| {
                        *data += 2;
                        _test = *data;
                    })
                    .unwrap();
                entity.remove_component::<u32>().unwrap();
                entity.add_component(32_u32).unwrap();
                entity.delete();
                world.create_entity();
            }
        });
    assert!(bool);
}
