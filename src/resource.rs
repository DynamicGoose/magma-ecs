use std::{any::{Any, TypeId}, collections::HashMap};

// TODO: change to Resources
#[derive(Default)]
pub struct Resource {
	data: HashMap<TypeId, Box<dyn Any>>,
}

impl Resource {
	pub fn add(&mut self, data: impl Any) {
		self.data.insert(data.type_id(), Box::new(data));
	}

	pub fn get_ref<T: Any>(&self) -> Option<&T> {
		let type_id = TypeId::of::<T>();
		if let Some(data) = self.data.get(&type_id) {
			data.downcast_ref()
		} else {
			None
		}
	}

	pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
		let type_id = TypeId::of::<T>();
		if let Some(data) = self.data.get_mut(&type_id) {
			data.downcast_mut()
		} else {
			None
		}
	}

	pub fn remove<T: Any>(&mut self) {
		let type_id = TypeId::of::<T>();
		self.data.remove(&type_id);
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn add_resource() {
		let resources = init_resource();

		let stored_resource = resources.data.get(&TypeId::of::<Time>()).unwrap();
		let extracted_time = stored_resource.downcast_ref::<Time>().unwrap();
		assert_eq!(extracted_time.0, 20);
	}

	#[test]
	fn get_resource() {
		let resources = init_resource();

		if let Some(extracted_time) = resources.get_ref::<Time>() {
			assert_eq!(extracted_time.0, 20);
		}
	}

	#[test]
	fn get_resource_mut() {
		let mut resources = init_resource();
		{
			let time: &mut Time = resources.get_mut::<Time>().unwrap();
			time.0 += 1;
		}
		let time = resources.get_ref::<Time>().unwrap();
		assert_eq!(time.0, 21);
	}

	#[test]
	fn remove_resource() {
		let mut resources = init_resource();
		resources.remove::<Time>();
		let time_type_id = TypeId::of::<Time>();
		assert!(!resources.data.contains_key(&time_type_id))
	}

	fn init_resource() -> Resource {
		let mut resources = Resource::default();
		let time = Time(20);

		resources.add(time);
		resources
	}

	struct Time(pub u64);
}