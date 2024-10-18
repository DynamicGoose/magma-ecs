use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::World;

use super::{System, Systems};

#[derive(Default)]
pub struct Dispatcher(Vec<Vec<fn(&World)>>);

impl Dispatcher {
    /// A system can't have shared dependencies with systems it depends on (the shared dependency is already guaranteed to have executed because of the other dependency).
    /// This function will lock when this happends. Improve in the future?
    pub(crate) fn from_systems(mut systems: Systems) -> Self {
        let mut in_dispatcher: Vec<System> = vec![];
        let mut dispatcher = Self::default();
        let mut stage: Vec<System>;

        while !systems.0.is_empty() {
            stage = systems
                .0
                .iter()
                .filter(|system| {
                    system
                        .deps
                        .iter()
                        .filter(|dep| {
                            !in_dispatcher
                                .iter()
                                .filter(|system| system.name == **dep)
                                .collect::<Vec<&System>>()
                                .is_empty()
                        })
                        .collect::<Vec<&&str>>()
                        .len()
                        == system.deps.len()
                })
                .cloned()
                .collect::<Vec<System>>();
            systems.0.retain(|system| !stage.contains(system));
            dispatcher
                .0
                .push(stage.iter().map(|system| system.run).collect());
            in_dispatcher.append(&mut stage);
        }
        dispatcher
    }

    pub fn dispatch(&self, world: &World) {
        self.0.iter().for_each(|systems| {
            systems.par_iter().for_each(|system| {
                (system)(world);
            })
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{systems::Systems, World};

    use super::Dispatcher;

    #[test]
    fn create_dispatcher() {
        let mut world = World::new();
        world.register_component::<u32>();

        let systems = Systems::new()
            .with(system_1, "system_1", &[])
            .with(system_2, "system_2", &["system_1"])
            .with(system_3, "system_3", &["system_1"])
            .with(system_4, "system_4", &["system_2", "system_3"]);
        let dispatcher = Dispatcher::from_systems(systems);
        (dispatcher.0[0][0])(&world);
        (dispatcher.0[1][0])(&world);
        world
            .query()
            .with_component::<u32>()
            .unwrap()
            .run(|entities| {
                entities[0]
                    .component_ref(|component: &u32| assert_eq!(*component, 1))
                    .unwrap();
                entities[1]
                    .component_ref(|component: &u32| assert_eq!(*component, 2))
                    .unwrap();
            });
    }

    #[test]
    fn dispatcher_dispatch() {
        let mut world = World::new();
        world.register_component::<u32>();

        let systems = Systems::new()
            .with(system_1, "system_1", &[])
            .with(system_2, "system_2", &["system_1"])
            .with(system_3, "system_3", &["system_1"])
            .with(system_4, "system_4", &["system_2", "system_3"]);
        let dispatcher = Dispatcher::from_systems(systems);

        dispatcher.dispatch(&world);

        world
            .query()
            .with_component::<u32>()
            .unwrap()
            .run(|entities| {
                entities[0]
                    .component_ref(|component: &u32| assert_eq!(*component, 2))
                    .unwrap();
                entities[3]
                    .component_ref(|component: &u32| assert_eq!(*component, 4))
                    .unwrap();
            });
    }

    fn system_1(world: &World) {
        world.create_entity().with_component(1_u32).unwrap();
    }
    fn system_2(world: &World) {
        world.create_entity().with_component(2_u32).unwrap();
    }
    fn system_3(world: &World) {
        world
            .query()
            .with_component::<u32>()
            .unwrap()
            .run(|entities| {
                entities
                    .iter()
                    .for_each(|entity| entity.component_mut(|comp: &mut u32| *comp += 1).unwrap())
            });
        world.create_entity().with_component(3_u32).unwrap();
    }
    fn system_4(world: &World) {
        world.create_entity().with_component(4_u32).unwrap();
    }
}
