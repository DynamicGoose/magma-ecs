use mecs::World;

#[test]
fn startup() {
    let mut world = World::new();
    world.register_component::<u32>();
    world.startup(vec![], vec![&create_u32_entity]);
    let query = world.query().with_component::<u32>().unwrap().run();
    assert_eq!(query.indexes.len(), 1);
}

#[test]
fn update() {
    let mut world = World::new();
    world.register_component::<u32>();
    world.update(&break_update_loop, vec![], vec![&create_u32_entity]);
    let query = world.query().with_component::<u32>().unwrap().run();
    assert_eq!(query.indexes.len(), 2);
}

fn break_update_loop(world: &World) -> bool {
    let query = world.query().with_component::<u32>().unwrap().run();
    query.indexes.len() != 2
}

fn create_u32_entity(world: &mut World) {
    world.spawn().with_component(10_u32).unwrap();
}
