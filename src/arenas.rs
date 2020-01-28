use crate::*;
use code_gen::{Field, Visibility, CamelCase, Struct, Derives, Impl, Function, CodeLine};
use std::convert::{TryInto, TryFrom};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Arena {
    pub name: CamelCase,
    pub allocator: Allocator,
    pub components: Vec<Component>,
    pub default_components: Vec<Component>,
}

impl Arena {
    pub fn fixed<E: Debug>(name: impl TryInto<CamelCase,Error=E>) -> Self {
        Arena {
            name: name.try_into().unwrap(),
            allocator: Allocator::Fixed,
            components: Default::default(),
            default_components: Default::default(),
        }
    }

    pub fn generational(name: impl TryInto<CamelCase,Error=String>) -> Self {
        Arena {
            name: name.try_into().unwrap(),
            allocator: Allocator::Generational,
            components: Default::default(),
            default_components: Default::default(),
        }
    }

    pub fn add_component(mut self, component: Component) -> Self {
        self.components.push(component);
        self
    }

    pub fn add_default_component(mut self, component: Component) -> Self {
        self.default_components.push(component);
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
            .chain(&self.default_components)
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
            .with_derives(Derives::with_debug())
            .with_fields(fields)
    }

    pub fn get_impl(&self) -> Impl {
        let mut insert = Function::new("insert")
            .with_parameters(&format!("&mut self, id: {}, row: {}", self.get_id_type(), self.get_data_row().get_type_string()));

        for component in self.components.iter() {
            let line = CodeLine::new(0, &format!("self.{}.insert(id, row.{});", component.name, component.name));
            insert = insert.add_line(line);
        }

        for component in self.default_components.iter() {
            let line = CodeLine::new(0, &format!("self.{}.insert(id, Default::default());", &component.name));
            insert = insert.add_line(line);
        }

        Impl::new(&self.get_struct())
            .add_function(insert)
    }

    pub fn get_id_type(&self) -> String {
        self.allocator.get_id_type(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arena_get_impl() {
        let arena = Arena::fixed("Arena")
            .add_component(Component::dense_from_type("Length"))
            .add_default_component(Component::dense_from_type("Width"));

        assert_eq!(
            "impl Arena {\n    pub fn insert(&mut self, id: Id<Arena>, row: ArenaRow) {\n        self.length.insert(id, row.length);\n        self.width.insert(id, Default::default());\n    }\n}\n",
            arena.get_impl().to_string()
        );
    }
}