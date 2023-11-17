use std::collections::HashMap;

pub fn push_entity(world: &mut HashMap<usize, ()>, components: (), id: Option<usize>) {
	world.insert(match id {
		None => world.len() + 1,
		// TODO: Check for duplicate ids
		Some(i) => i,
	}, components);
}