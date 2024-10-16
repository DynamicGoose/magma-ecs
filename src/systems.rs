use crate::World;

struct SystemMut {
    run: fn(&mut World),
    name: &'static str,
    deps: &'static [&'static str],
}

struct SystemRef {
    run: fn(&World),
    name: &'static str,
    deps: &'static [&'static str],
}

pub struct Systems {
    mutable: Vec<SystemMut>,
    immutable: Vec<SystemRef>,
}

impl Systems {
    pub fn build_from(
        systems_mut: &[(fn(&mut World), &'static str, &'static [&'static str])],
        systems_ref: &[(fn(&World), &'static str, &'static [&'static str])],
    ) -> Self {
        Self {
            mutable: systems_mut
                .iter()
                .map(|system| SystemMut {
                    run: system.0,
                    name: system.1,
                    deps: system.2,
                })
                .collect(),
            immutable: systems_ref
                .iter()
                .map(|system| SystemRef {
                    run: system.0,
                    name: system.1,
                    deps: system.2,
                })
                .collect(),
        }
    }
}
