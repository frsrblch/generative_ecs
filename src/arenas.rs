use crate::*;
use code_gen::{Field, Visibility, CamelCase};

#[derive(Debug)]
pub struct Arena {
    pub name: CamelCase,
    pub allocator: Allocator,
    pub components: Vec<Component>,
}

impl Arena {
    pub fn get_allocator_field(&self) -> Field {
        Field {
            visibility: Visibility::Pub,
            name: self.name.clone().into(),
            field_type: self.allocator.get_type(self),
        }
    }

    pub fn get_state_field(&self) -> Field {
        Field {
            visibility: Visibility::Pub,
            name: self.name.clone().into(),
            field_type: self.name.to_string(),
        }
    }
}
