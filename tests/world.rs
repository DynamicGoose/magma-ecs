use magma_ecs::World;

#[test]
fn update() {
    let world = World::new();
    world.register_component::<u32>();
    world.update(vec![create_u32_entity, create_u32_entity]);

    let entities = world.entities_read();
    let query = entities.query().with_component::<u32>().unwrap().run();
    assert_eq!(query.indexes.len(), 2);
}

fn create_u32_entity(world: &World) {
    {
        let mut entities = world.entities_write();
        entities.create_entity().with_component(10_u32).unwrap();
    }

    let entities = world.entities_read();
    let mut query = entities.query();
    let entities = query.with_component::<u32>().unwrap().run_entity();

    for entity in entities {
        *entity.get_component_mut::<u32>().unwrap().downcast_mut::<u32>().unwrap() += 10;
    }
}
