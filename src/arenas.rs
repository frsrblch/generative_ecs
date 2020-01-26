use crate::*;
use code_gen::{Field, Visibility, CamelCase, Struct, Derives};
use std::convert::{TryInto, TryFrom};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Arena {
    pub name: CamelCase,
    pub allocator: Allocator,
    pub components: Vec<Component>,
}

impl Arena {
    pub fn fixed<E: Debug>(name: impl TryInto<CamelCase,Error=E>) -> Self {
        Arena {
            name: name.try_into().unwrap(),
            allocator: Allocator::Fixed,
            components: Default::default(),
        }
    }

    pub fn generational(name: impl TryInto<CamelCase,Error=String>) -> Self {
        Arena {
            name: name.try_into().unwrap(),
            allocator: Allocator::Generational,
            components: Default::default(),
        }
    }

    pub fn add_component(mut self, component: Component) -> Self {
        self.components.push(component);
        self
    }

    pub fn get_allocator_field(&self) -> Field {
        Field {
            visibility: Visibility::Pub,
            name: self.name.clone().into(),
            field_type: self.allocator.get_type(self),
        }
    }

    pub fn get_state_field(&self) -> Field {
        Field::new(self.name.clone(), &self.name)
    }

    pub fn get_struct(&self) -> Struct {
        let fields = self.components.iter()
            .map(Component::get_arena_field)
            .collect();

        Struct::new(self.name.clone())
            .with_derives(Derives::with_debug_default_clone())
            .with_fields(fields)
    }

    pub fn get_data_row(&self) -> Struct {
        let mut name = self.name.to_string();
        name += "Row";
        let name = CamelCase::try_from(name.as_str()).unwrap();

        let fields = self.components.iter()
            .map(Component::get_data_field)
            .collect();

        Struct::new(name)
            .with_derives(Derives::with_debug_default_clone())
            .with_fields(fields)
    }
}
