use magma_ecs::World;

#[test]
fn create_and_get_resource_immut() {
    let world = init_world();

    let resources = world.resources_read();
    let fps = resources.get_ref::<FpsResource>().unwrap();
    assert_eq!(fps.0, 60.0);
}

#[test]
fn get_resource_mut() {
    let world = init_world();
    {
        let mut resources = world.resources_write();
        let fps: &mut FpsResource = resources.get_mut::<FpsResource>().unwrap();
        fps.0 += 1.0;
    }
    let resources = world.resources_read();
    let fps = resources.get_ref::<FpsResource>().unwrap();
    assert_eq!(fps.0, 61.0);
}

#[test]
fn remove_resource() {
    let world = init_world();
    world.remove_resource::<FpsResource>();

    let resources = world.resources_read();
    let deleted_resource = resources.get_ref::<FpsResource>();
    assert!(deleted_resource.is_none());
}

fn init_world() -> World {
    let world = World::new();
    world.add_resource(FpsResource(60.0));
    world
}

#[derive(PartialEq, Debug)]
struct FpsResource(pub f64);

impl std::ops::Deref for FpsResource {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
