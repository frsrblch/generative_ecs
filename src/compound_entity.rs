use code_gen::{CamelCase, Struct, Derives, Field};
use crate::{Arena, World};

#[derive(Debug, Clone)]
pub struct CompoundEntity {
    pub name: CamelCase,
    pub primary_arena: CamelCase,
    pub child_arenas: Vec<CamelCase>,
}

impl CompoundEntity {
    pub fn new(name: &str, primary_arena: &Arena, child_arenas: Vec<&Arena>) -> Self {
        CompoundEntity {
            name: name.parse().unwrap(),
            primary_arena: primary_arena.name.clone(),
            child_arenas: child_arenas.into_iter().map(|a| a.name.clone()).collect(),
        }
    }

    pub fn get_struct(&self, world: &World) -> Struct {
        let arena = world.get_arena(&self.primary_arena);

        let s = Struct::new(self.name.as_str())
            .with_derives(Derives::with_debug_clone());

        for (r, l) in arena.references.iter() {
            unimplemented!()
        }

        for child in self.child_arenas.iter() {
            let child = world.get_arena(child);

            unimplemented!()
        }

        unimplemented!()
    }
}