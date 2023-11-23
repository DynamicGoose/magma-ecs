use mecs::World;

#[test]
fn create_and_get_resource_immut() {
	let world = init_world();

	let fps = world.get_resource::<FpsResource>().unwrap();
	assert_eq!(fps.0, 60.0);
}

#[test]
fn get_resource_mut() {
	let mut world = init_world();
	{
		let fps: &mut FpsResource = world.get_resource_mut::<FpsResource>().unwrap();
		fps.0 += 1.0;
	}
	let fps = world.get_resource::<FpsResource>().unwrap();
	assert_eq!(fps.0, 61.0);
}

#[test]
fn remove_resource() {
	let mut world = init_world();
	world.remove_resource::<FpsResource>();
	let deleted_resource = world.get_resource::<FpsResource>();
	assert!(deleted_resource.is_none());
}

fn init_world() -> World {
	let mut world = World::new();
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