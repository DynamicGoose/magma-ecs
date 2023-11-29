use std::any::{Any, TypeId};

use crate::errors::MecsErrors;

use super::{query_result::QueryResult, Entities};

#[derive(Debug)]
pub struct Query<'a> {
    map: u128,
    entities: &'a Entities,
    type_ids: Vec<TypeId>,
}

impl<'a> Query<'a> {
    pub fn new(entities: &'a Entities) -> Self {
        Self {
            entities,
            map: 0,
            type_ids: vec![],
        }
    }

    pub fn with_component<T: Any>(&mut self) -> Result<&mut Self, MecsErrors> {
        let type_id = TypeId::of::<T>();
        if let Some(bit_mask) = self.entities.get_bitmask(&type_id) {
            self.map |= bit_mask;
            self.type_ids.push(type_id);
        } else {
            return Err(MecsErrors::ComponentNotRegistered);
        }
        Ok(self)
    }

    pub fn run(&self) -> QueryResult {
        let indexes: Vec<usize> = self
            .entities
            .map
            .iter()
            .enumerate()
            .filter_map(|(index, entity_map)| {
                if entity_map & self.map == self.map {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        let mut components = vec![];

        for type_id in &self.type_ids {
            let entity_components = self.entities.components.get(type_id).unwrap();
            let mut components_to_keep = vec![];
            for index in &indexes {
                components_to_keep.push(entity_components[*index].clone().unwrap());
            }
            components.push(components_to_keep)
        }

        QueryResult {
            indexes,
            components,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn query_with_component() {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();
        let mut query = Query::new(&entities);
        query
            .with_component::<u32>()
            .unwrap()
            .with_component::<f32>()
            .unwrap();

        assert!(
            query.map == 3
                && TypeId::of::<u32>() == query.type_ids[0]
                && TypeId::of::<f32>() == query.type_ids[1]
        );
    }

    #[test]
    fn run_query() {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();
        entities
            .create_entity()
            .with_component(10_u32)
            .unwrap()
            .with_component(20.0_f32)
            .unwrap();
        entities.create_entity().with_component(5_u32).unwrap();
        entities.create_entity().with_component(20.0_f32).unwrap();
        entities
            .create_entity()
            .with_component(15_u32)
            .unwrap()
            .with_component(25.0_f32)
            .unwrap();
        let mut query = Query::new(&entities);
        query
            .with_component::<u32>()
            .unwrap()
            .with_component::<f32>()
            .unwrap();

        let query_result = query.run();
        let u32s = &query_result.components[0];
        dbg!(u32s);
        let f32s = &query_result.components[1];
        dbg!(f32s);
        let indexes = &query_result.indexes;
        dbg!(indexes);

        let first_u32 = *u32s[0].borrow().downcast_ref::<u32>().unwrap();
        dbg!(first_u32);

        assert!(
            u32s.len() == f32s.len()
                && u32s.len() == 2
                && first_u32 == 10
                && u32s.len() == indexes.len()
                && indexes[0] == 0
                && indexes[1] == 3
        );
    }
}
