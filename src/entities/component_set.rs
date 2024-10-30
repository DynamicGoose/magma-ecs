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

impl<C0, C1, C2, C3, C4> ComponentSet for (C0, C1, C2, C3, C4)
where
    C0: Any + Send + Sync,
    C1: Any + Send + Sync,
    C2: Any + Send + Sync,
    C3: Any + Send + Sync,
    C4: Any + Send + Sync,
{
    fn for_components<R: FnMut(TypeId, Arc<RwLock<dyn Any + Send + Sync>>)>(self, mut run: R) {
        run(self.0.type_id(), Arc::new(RwLock::new(self.0)));
        run(self.1.type_id(), Arc::new(RwLock::new(self.1)));
        run(self.2.type_id(), Arc::new(RwLock::new(self.2)));
        run(self.3.type_id(), Arc::new(RwLock::new(self.3)));
        run(self.4.type_id(), Arc::new(RwLock::new(self.4)));
    }
}

impl<C0, C1, C2, C3, C4, C5> ComponentSet for (C0, C1, C2, C3, C4, C5)
where
    C0: Any + Send + Sync,
    C1: Any + Send + Sync,
    C2: Any + Send + Sync,
    C3: Any + Send + Sync,
    C4: Any + Send + Sync,
    C5: Any + Send + Sync,
{
    fn for_components<R: FnMut(TypeId, Arc<RwLock<dyn Any + Send + Sync>>)>(self, mut run: R) {
        run(self.0.type_id(), Arc::new(RwLock::new(self.0)));
        run(self.1.type_id(), Arc::new(RwLock::new(self.1)));
        run(self.2.type_id(), Arc::new(RwLock::new(self.2)));
        run(self.3.type_id(), Arc::new(RwLock::new(self.3)));
        run(self.4.type_id(), Arc::new(RwLock::new(self.4)));
        run(self.5.type_id(), Arc::new(RwLock::new(self.5)));
    }
}

impl<C0, C1, C2, C3, C4, C5, C6> ComponentSet for (C0, C1, C2, C3, C4, C5, C6)
where
    C0: Any + Send + Sync,
    C1: Any + Send + Sync,
    C2: Any + Send + Sync,
    C3: Any + Send + Sync,
    C4: Any + Send + Sync,
    C5: Any + Send + Sync,
    C6: Any + Send + Sync,
{
    fn for_components<R: FnMut(TypeId, Arc<RwLock<dyn Any + Send + Sync>>)>(self, mut run: R) {
        run(self.0.type_id(), Arc::new(RwLock::new(self.0)));
        run(self.1.type_id(), Arc::new(RwLock::new(self.1)));
        run(self.2.type_id(), Arc::new(RwLock::new(self.2)));
        run(self.3.type_id(), Arc::new(RwLock::new(self.3)));
        run(self.4.type_id(), Arc::new(RwLock::new(self.4)));
        run(self.5.type_id(), Arc::new(RwLock::new(self.5)));
        run(self.6.type_id(), Arc::new(RwLock::new(self.6)));
    }
}

impl<C0, C1, C2, C3, C4, C5, C6, C7> ComponentSet for (C0, C1, C2, C3, C4, C5, C6, C7)
where
    C0: Any + Send + Sync,
    C1: Any + Send + Sync,
    C2: Any + Send + Sync,
    C3: Any + Send + Sync,
    C4: Any + Send + Sync,
    C5: Any + Send + Sync,
    C6: Any + Send + Sync,
    C7: Any + Send + Sync,
{
    fn for_components<R: FnMut(TypeId, Arc<RwLock<dyn Any + Send + Sync>>)>(self, mut run: R) {
        run(self.0.type_id(), Arc::new(RwLock::new(self.0)));
        run(self.1.type_id(), Arc::new(RwLock::new(self.1)));
        run(self.2.type_id(), Arc::new(RwLock::new(self.2)));
        run(self.3.type_id(), Arc::new(RwLock::new(self.3)));
        run(self.4.type_id(), Arc::new(RwLock::new(self.4)));
        run(self.5.type_id(), Arc::new(RwLock::new(self.5)));
        run(self.6.type_id(), Arc::new(RwLock::new(self.6)));
        run(self.7.type_id(), Arc::new(RwLock::new(self.7)));
    }
}

impl<C0, C1, C2, C3, C4, C5, C6, C7, C8> ComponentSet for (C0, C1, C2, C3, C4, C5, C6, C7, C8)
where
    C0: Any + Send + Sync,
    C1: Any + Send + Sync,
    C2: Any + Send + Sync,
    C3: Any + Send + Sync,
    C4: Any + Send + Sync,
    C5: Any + Send + Sync,
    C6: Any + Send + Sync,
    C7: Any + Send + Sync,
    C8: Any + Send + Sync,
{
    fn for_components<R: FnMut(TypeId, Arc<RwLock<dyn Any + Send + Sync>>)>(self, mut run: R) {
        run(self.0.type_id(), Arc::new(RwLock::new(self.0)));
        run(self.1.type_id(), Arc::new(RwLock::new(self.1)));
        run(self.2.type_id(), Arc::new(RwLock::new(self.2)));
        run(self.3.type_id(), Arc::new(RwLock::new(self.3)));
        run(self.4.type_id(), Arc::new(RwLock::new(self.4)));
        run(self.5.type_id(), Arc::new(RwLock::new(self.5)));
        run(self.6.type_id(), Arc::new(RwLock::new(self.6)));
        run(self.7.type_id(), Arc::new(RwLock::new(self.7)));
        run(self.8.type_id(), Arc::new(RwLock::new(self.8)));
    }
}

impl<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9> ComponentSet
    for (C0, C1, C2, C3, C4, C5, C6, C7, C8, C9)
where
    C0: Any + Send + Sync,
    C1: Any + Send + Sync,
    C2: Any + Send + Sync,
    C3: Any + Send + Sync,
    C4: Any + Send + Sync,
    C5: Any + Send + Sync,
    C6: Any + Send + Sync,
    C7: Any + Send + Sync,
    C8: Any + Send + Sync,
    C9: Any + Send + Sync,
{
    fn for_components<R: FnMut(TypeId, Arc<RwLock<dyn Any + Send + Sync>>)>(self, mut run: R) {
        run(self.0.type_id(), Arc::new(RwLock::new(self.0)));
        run(self.1.type_id(), Arc::new(RwLock::new(self.1)));
        run(self.2.type_id(), Arc::new(RwLock::new(self.2)));
        run(self.3.type_id(), Arc::new(RwLock::new(self.3)));
        run(self.4.type_id(), Arc::new(RwLock::new(self.4)));
        run(self.5.type_id(), Arc::new(RwLock::new(self.5)));
        run(self.6.type_id(), Arc::new(RwLock::new(self.6)));
        run(self.7.type_id(), Arc::new(RwLock::new(self.7)));
        run(self.8.type_id(), Arc::new(RwLock::new(self.8)));
        run(self.9.type_id(), Arc::new(RwLock::new(self.9)));
    }
}
