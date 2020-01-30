use crate::*;
use code_gen::{Field, Visibility, CamelCase, Struct, Derives, Impl, Function, CodeLine, Type};
use std::fmt::Debug;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Arena {
    pub name: CamelCase,
    pub allocator: Allocator,
    pub components: Vec<Component>,
    pub default_components: Vec<Component>,
    pub references: HashMap<CamelCase, LinkType>,
    pub ownership: HashMap<CamelCase, LinkType>,
}

//	From	    To	        Relationsh	Use Case	                                        Example
//	Permanent	Permanent	Owns	    A, B, C -> D	                                    shared component
//	Permanent	Permanent	Maybe Owns	A -> Opt<B>	not all bodies have an atmosphere
//	Permanent	Permanent	Ref     	A -- B	                                            all bodies reference a system
//	Permanent	Permanent	Maybe Ref	A -- Opt<B>	                                        ??
//	Permanent	Transient	Owns	    INVALID, cannot be unlinked if child removed	    -
//	Permanent	Transient	Maybe Owns	A -> Opt<B>	                                        ??
//	Permanent	Transient	Ref	        INVALID, cannot be unlinked if child removed	    -
//	Permanent	Transient	Maybe Ref	A -- Opt<B>	                                        ??
//	Transient	Permanent	Owns	    INVALID, child entity will leak if parent removed	-
//	Transient	Permanent	Maybe Owns	INVALID, child entity will leak if parent removed	-
//	Transient	Permanent	Ref	        A -- B	                                            colony references the body it's built upon
//	Transient	Permanent	Maybe Ref	A -- Opt<B>	                                        ships can reference a system, but may not be in one
//	Transient	Transient	Owns	    A, B, C -> D	                                    shared component, only deleted with the owner
//	Transient	Transient	Maybe Owns	A -> Opt<B>	                                        optional or shared component, only deleted by the owner
//	Transient	Transient	Ref	        INVALID, cannot be unlinked if child removed	    must point at owner, so refer is deleted along with it
//	Transient	Transient	Maybe Ref	A -- Opt<B>                                         ship refers to its controller

impl Arena {
    pub fn fixed(name: &str) -> Self {
        Arena {
            name: name.parse().unwrap(),
            allocator: Allocator::Fixed,
            components: Default::default(),
            default_components: Default::default(),
            references: Default::default(),
            ownership: Default::default(),
        }
    }

    pub fn generational(name: &str) -> Self {
        Arena {
            name: name.parse().unwrap(),
            allocator: Allocator::Generational,
            components: Default::default(),
            default_components: Default::default(),
            references: Default::default(),
            ownership: Default::default(),
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

    pub fn add_reference(mut self, reference_to: &Arena, link_type: LinkType) -> Self {
        self.references.insert(reference_to.name.clone(), link_type);
        self
    }

    pub fn add_ownership(mut self, owned: &Arena, link_type: LinkType) -> Self {
        self.ownership.insert(owned.name.clone(), link_type);
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
        Field::new(self.name.clone().into(), self.name.clone().into())
    }

    pub fn get_struct(&self) -> Struct {
        let fields = self.components.iter()
            .chain(&self.default_components)
            .map(Component::get_arena_field)
            .collect();

        Struct {
            typ: self.name.clone().into(),
            visibility: Default::default(),
            derives: Derives::with_debug_default_clone(),
            fields,
        }
    }

    pub fn get_data_row(&self) -> Struct {
        let mut name = self.name.to_string();
        name += "Row";
        let name = CamelCase::from_str(name.as_str()).unwrap();

        let fields = self.components.iter()
            .map(Component::get_data_field)
            .collect();

        Struct::new(name.as_str())
            .with_derives(Derives::with_debug())
            .with_fields(fields)
    }

    pub fn get_impl(&self) -> Impl {
        let id = match self.allocator {
            Allocator::Fixed => String::from("Id<Self>"),
            Allocator::Generational => String::from("Valid<Self>"),
        };
        let data_row = self.get_data_row().typ;
        let allocator = match self.allocator {
            Allocator::Fixed => String::from("FixedAllocator<Self>"),
            Allocator::Generational => String::from("GenAllocator<Self>"),
        };

        let mut insert = Function::new("insert")
            .with_parameters(&format!("&mut self, id: {}, row: {}", id, data_row));

        for component in self.components.iter() {
            let line = CodeLine::new(0, &format!("self.{}.insert(id, row.{});", component.name, component.name));
            insert = insert.add_line(line);
        }

        for component in self.default_components.iter() {
            let line = CodeLine::new(0, &format!("self.{}.insert(id, Default::default());", &component.name));
            insert = insert.add_line(line);
        }

        let create = Function::new("create")
            .with_parameters(&format!(
                "&mut self, row: {row}, allocator: &mut {al}",
                row=data_row,
                al=allocator,
            ))
            .with_return(self.get_id_type().to_string())
            .add_line(CodeLine::new(0, "let id = allocator.create();"))
            .add_line(CodeLine::new(0, "self.insert(id, row);"))
            .add_line(CodeLine::new(0, "id"));

        Impl::new(self.get_struct().typ)
            .add_function(insert)
            .add_function(create)
    }

    pub fn get_id_type(&self) -> Type {
        self.allocator.get_id_type(&self)
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn arena_get_impl() {
//        let arena = Arena::fixed("Arena")
//            .add_component(Component::dense_from_type("Length"))
//            .add_default_component(Component::dense_from_type("Width"));
//
//        assert_eq!(
//            "impl Arena {\n    pub fn insert(&mut self, id: Id<Self>, row: ArenaRow) {\n        self.length.insert(id, row.length);\n        self.width.insert(id, Default::default());\n    }\n}\n",
//            arena.get_impl().to_string()
//        );
//    }
//}