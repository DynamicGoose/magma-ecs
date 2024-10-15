use magma_ecs::{entities::query::Query, World};

#[test]
fn create_entity() {
    let world = World::new();
    world.register_component::<u64>();

    world.create_entity().with_component(400_u64).unwrap();
}

#[test]
fn query() {
    let world = World::new();
    world.register_component::<u32>();
    world.create_entity().with_component(32_u32).unwrap();

    let mut bool = false;
    let entities = world.entities_read();
    Query::new(&entities)
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
}
