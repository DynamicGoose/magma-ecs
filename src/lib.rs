pub mod world;
pub mod entity;
mod component;


#[cfg(test)]
mod tests {
    use crate::world::World;
    use crate::entity::*;

    struct Health(i32);
    struct Name(&'static str);

    #[test]
    fn world_test() {
        let mut world = World::new();
        let entity = world.new_entity();

        world.add_component_to_entity(entity, Name("Name"));
        world.add_component_to_entity(entity, Health(-10));

        let new_entity = world.new_entity();

        let mut healths = world.borrow_component_vec_mut::<Health>().unwrap();
        let mut names = world.borrow_component_vec_mut::<Name>().unwrap();

        assert_eq!(new_entity, 1);
    }
}