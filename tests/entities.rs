use magma_ecs::World;

#[test]
fn create_entity() {
    let mut world = World::new();
    world.register_component::<u64>();

    world.create_entity((400_u64,)).unwrap();
}

#[test]
fn query() {
    let mut world = World::new();
    world.register_component::<u32>();
    world.create_entity((32_u32,)).unwrap();

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
                world.create_entity((32_u32,)).unwrap();
            }
        });
    assert!(bool);
}

#[test]
fn inner_lock() {
    let mut world = World::new();
    world.register_component::<u32>();
    world.register_component::<f32>();
    world.create_entity((32_u32, 64.0_f32)).unwrap();

    world
        .query()
        .with_component::<u32>()
        .unwrap()
        .run(|entities| {
            for entity in entities {
                world
                    .query()
                    .with_component::<f32>()
                    .unwrap()
                    .run(|entities_2| {
                        for entity_2 in entities_2 {
                            entity_2
                                .component_mut(|comp_mut: &mut f32| {
                                    entity
                                        .component_ref(|comp_ref: &u32| {
                                            if *comp_ref < *comp_mut as u32 {
                                                *comp_mut += 1.0;
                                            }
                                        })
                                        .unwrap();
                                })
                                .unwrap();
                        }
                    });
            }
        });
}
