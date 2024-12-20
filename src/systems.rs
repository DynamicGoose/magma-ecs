use dispatcher::Dispatcher;

use crate::World;

/// The [`Dispatcher`] is used to dispatch [`Systems`] in parallel on a [`World`].
pub mod dispatcher;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct System {
    pub run: fn(&World),
    pub name: &'static str,
    pub deps: &'static [&'static str],
}

impl System {
    fn new(run: fn(&World), name: &'static str, deps: &'static [&'static str]) -> Self {
        Self { run, name, deps }
    }
}

/// Holds systems and their dependencies
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Systems(pub(crate) Vec<System>);

impl Systems {
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Add a system on creation.
    /// **run:** the system's function,
    /// **name:** the system's name,
    /// **deps:** the system's dependencies,
    pub fn with(
        mut self,
        run: fn(&World),
        name: &'static str,
        deps: &'static [&'static str],
    ) -> Self {
        self.0.push(System::new(run, name, deps));
        self
    }

    /// Add a system
    pub fn add(
        &mut self,
        run: fn(&World),
        name: &'static str,
        deps: &'static [&'static str],
    ) -> &mut Self {
        self.0.push(System::new(run, name, deps));
        self
    }

    /// Build a [`Dispatcher`] from the [`Systems`] to be run on the [`World`].
    pub fn build_dispatcher(self) -> Dispatcher {
        Dispatcher::from_systems(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::World;

    use super::Systems;

    #[test]
    fn create_systems() {
        let systems = Systems::new().with(system_1, "system_1", &[]).with(
            system_2,
            "system_2",
            &["system_1"],
        );
        assert_eq!(systems.0[1].name, "system_2");
    }

    #[test]
    fn add_systems() {
        let mut systems = Systems::new();
        systems
            .add(system_1, "system_1", &[])
            .add(system_2, "system_2", &["system_1"]);
    }

    #[test]
    fn build_dispatcher() {
        let systems = Systems::new().with(system_1, "system_1", &[]).with(
            system_2,
            "system_2",
            &["system_1"],
        );
        systems.build_dispatcher();
    }

    fn system_1(_: &World) {}
    fn system_2(_: &World) {}
}
