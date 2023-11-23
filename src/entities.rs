use std::{any::{Any, TypeId}, rc::Rc, cell::RefCell, collections::HashMap};

// TODO: Implement better API
#[derive(Debug, Default)]
pub struct Entities {
	components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
}

impl Entities {
	pub fn register_component<T: Any>(&mut self) {
		self.components.insert(TypeId::of::<T>(), vec![]);
	}
}

#[cfg(test)]
mod test {
	use std::any::TypeId;

use super::*;

	#[test]
	fn register_entity() {
		let mut entities = Entities::default();
		entities.register_component::<Health>();
		let type_id = TypeId::of::<Health>();
		let health_components = entities.components.get(&type_id).unwrap();
		assert_eq!(health_components.len(), 0);
	}

	struct Health(u32);
}