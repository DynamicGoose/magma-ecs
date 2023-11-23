use std::any::Any;

use resource::Resource;

mod resource;

#[derive(Default)]
pub struct World {
	resources: Resource,
}

impl World {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn add_resource(&mut self, resource_data: impl Any) {
		self.resources.add(resource_data);
	}

	pub fn get_resource<T: Any>(&self) -> Option<&T> {
		self.resources.get_ref::<T>()
	}

	pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
		self.resources.get_mut::<T>()
	}
}

#[cfg(test)]
mod tests {}