//! provides the [`Resources`] struct.
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Default)]
pub struct Resources {
    data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Resources {
    pub(crate) fn add(&mut self, data: impl Any + Send + Sync) {
        self.data.insert(data.type_id(), Box::new(data));
    }

    pub fn get_ref<T: Any + Send + Sync>(&self) -> Option<&T> {
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

    pub(crate) fn remove<T: Any>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.data.remove(&type_id);
    }
}

#[cfg(test)]
mod test {}
