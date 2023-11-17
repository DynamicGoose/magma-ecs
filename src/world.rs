use std::collections::HashMap;

use crate::entity::push_entity;

/// World
pub struct World(HashMap<usize, ()>);

impl World {
	pub fn new() -> Self {
		Self(HashMap::new())
	}
	pub fn push(&mut self, components: (), id: Option<usize>) {
		push_entity(&mut self.0, components, id)
	}
}