struct World {
    entities: u32,
    components: Vec<Vec<u32>>,
}

impl World {
    pub fn new() -> Self {
        Self { entities: 0, components: Vec::new }
    }

    pub fn add_entity(&mut self) -> u32 {
        let entity_id = self.entities;
        
        for component in self.components {
            component.push(0);
        }

        self.entities += 1;
        self.entities
    }
}