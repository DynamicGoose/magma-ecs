use magma_ecs::{systems::Systems, World};

#[test]
fn create_systems() {
    Systems::new()
        .with(system_1, "system_1", &[])
        .with(system_2, "system_2", &["system_1"])
        .with(system_3, "system_3", &["system_1"])
        .with(system_4, "system_4", &["system_2", "system_3"]);
}

#[test]
fn dispatcher() {
    let mut world = World::new();
    world.register_component::<u32>();

    let systems = Systems::new()
        .with(system_1, "system_1", &[])
        .with(system_2, "system_2", &["system_1"])
        .with(system_3, "system_3", &["system_1"])
        .with(system_4, "system_4", &["system_2", "system_3"]);
    let dispatcher = systems.build_dispatcher();
    dispatcher.dispatch(&world);

    world
        .query()
        .with_component::<u32>()
        .unwrap()
        .run(|entities| {
            entities[0]
                .component_ref(|component: &u32| assert_eq!(*component, 2))
                .unwrap();
            entities[3]
                .component_ref(|component: &u32| assert_eq!(*component, 4))
                .unwrap();
        });
}

// test systems
fn system_1(world: &World) {
    world.create_entity().with_component(1_u32).unwrap();
}
fn system_2(world: &World) {
    world.create_entity().with_component(2_u32).unwrap();
}
fn system_3(world: &World) {
    world
        .query()
        .with_component::<u32>()
        .unwrap()
        .run(|entities| {
            entities
                .iter()
                .for_each(|entity| entity.component_mut(|comp: &mut u32| *comp += 1).unwrap())
        });
    world.create_entity().with_component(3_u32).unwrap();
}
fn system_4(world: &World) {
    world.create_entity().with_component(4_u32).unwrap();
}
