trait ComponentVec {
    fn push_none(&mut self);
}

impl<T: 'static> ComponentVec for Vec<Option<T>>  {
    fn push_none(&mut self) {
        self.push(None);
    }
}
struct Scene {
    entities: u32,
    components: Vec<Box<dyn ComponentVec>>,
}

impl Scene {
    pub fn new() -> Self {
        Self { entities: 0, components: Vec::new() }
    }

    pub fn entity(&mut self) -> u32 {
        let entity_id = self.entities;
        for component in self.components.iter_mut() {
            component.push_none();
        }
        self.entities += 1;
        entity_id
    }
}