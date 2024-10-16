use std::sync::{Arc, Condvar, Mutex};

use crate::World;

pub(crate) struct SystemMut {
    pub run: fn(&mut World),
    pub name: &'static str,
    pub cv: Arc<(Mutex<bool>, Condvar)>,
}

pub(crate) struct SystemRef {
    pub run: fn(&World),
    pub name: &'static str,
    pub cv: Arc<(Mutex<bool>, Condvar)>,
}

#[derive(Default)]
pub struct Systems {
    pub(crate) immutable: Vec<SystemRef>,
    pub(crate) mutable: Vec<SystemMut>,
}

impl Systems {
    pub fn new() -> Self {
        Self {
            immutable: vec![],
            mutable: vec![],
        }
    }

    pub fn add_system_ref(&mut self, system: fn(&World), name: &'static str) {
        self.immutable.push(SystemRef {
            run: system,
            name,
            cv: Arc::new((Mutex::new(false), Condvar::new())),
        });
    }

    pub fn add_system_mut(&mut self, system: fn(&mut World), name: &'static str) {
        self.mutable.push(SystemMut {
            run: system,
            name,
            cv: Arc::new((Mutex::new(false), Condvar::new())),
        });
    }
}
