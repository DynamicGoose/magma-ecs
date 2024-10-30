use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use parking_lot::RwLock;

pub trait ComponentSet {
    fn for_components<R: FnMut(TypeId, Arc<RwLock<dyn Any + Send + Sync>>)>(self, run: R);
}

impl ComponentSet for () {
    fn for_components<R: FnMut(TypeId, Arc<RwLock<dyn Any + Send + Sync>>)>(self, _run: R) {}
}

impl<C0> ComponentSet for (C0,)
where
    C0: Any + Send + Sync,
{
    fn for_components<R: FnMut(TypeId, Arc<RwLock<dyn Any + Send + Sync>>)>(self, mut run: R) {
        run(self.0.type_id(), Arc::new(RwLock::new(self.0)));
    }
}

impl<C0, C1> ComponentSet for (C0, C1)
where
    C0: Any + Send + Sync,
    C1: Any + Send + Sync,
{
    fn for_components<R: FnMut(TypeId, Arc<RwLock<dyn Any + Send + Sync>>)>(self, mut run: R) {
        run(self.0.type_id(), Arc::new(RwLock::new(self.0)));
        run(self.1.type_id(), Arc::new(RwLock::new(self.1)));
    }
}

impl<C0, C1, C2> ComponentSet for (C0, C1, C2)
where
    C0: Any + Send + Sync,
    C1: Any + Send + Sync,
    C2: Any + Send + Sync,
{
    fn for_components<R: FnMut(TypeId, Arc<RwLock<dyn Any + Send + Sync>>)>(self, mut run: R) {
        run(self.0.type_id(), Arc::new(RwLock::new(self.0)));
        run(self.1.type_id(), Arc::new(RwLock::new(self.1)));
        run(self.2.type_id(), Arc::new(RwLock::new(self.2)));
    }
}

impl<C0, C1, C2, C3> ComponentSet for (C0, C1, C2, C3)
where
    C0: Any + Send + Sync,
    C1: Any + Send + Sync,
    C2: Any + Send + Sync,
    C3: Any + Send + Sync,
{
    fn for_components<R: FnMut(TypeId, Arc<RwLock<dyn Any + Send + Sync>>)>(self, mut run: R) {
        run(self.0.type_id(), Arc::new(RwLock::new(self.0)));
        run(self.1.type_id(), Arc::new(RwLock::new(self.1)));
        run(self.2.type_id(), Arc::new(RwLock::new(self.2)));
        run(self.3.type_id(), Arc::new(RwLock::new(self.3)));
    }
}
