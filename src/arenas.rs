use crate::*;
use code_gen::{Field, Visibility, CamelCase, Struct, Derives, Impl, Function, CodeLine, Type, SnakeCase};
use std::fmt::Debug;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Arena {
    pub name: CamelCase,
    pub allocator: Allocator,
    pub components: Vec<ComponentType>,
    pub default_components: Vec<ComponentType>,
    pub references: HashMap<CamelCase, LinkType>,
    pub ownership: HashMap<CamelCase, LinkType>,
}

//	From	    To	        Relationsh	Use Case	                                        Example
//	Permanent	Permanent	Owns	    A, B, C -> D	                                    shared component
//	Permanent	Permanent	MaybeOwns	A -> Opt<B>	                                        not all bodies have an atmosphere
//	Permanent	Permanent	ManyOwns	A -> [B]	                                        planet owns several mine sites
//	Permanent	Permanent	Ref     	A -- B	                                            all bodies reference a system
//	Permanent	Permanent	MaybeRef	A -- Opt<B>	                                        ??
//	Permanent	Permanent	ManyRef	    A -- [B]	                                        NEW
//	Permanent	Transient	Owns	    INVALID, no reason for child to be transient	    -
//	Permanent	Transient	MaybeOwns	A -> Opt<B>	                                        ??
//	Permanent	Transient	ManyOwns	A -> [B]	                                        NEW
//	Permanent	Transient	Ref	        INVALID, cannot be unlinked if child removed	    -
//	Permanent	Transient	MaybeRef	A -- Opt<B>	                                        ??
//	Permanent	Transient	ManyRef 	A -- [B]	                                        Systems lists bodies contained
//	Transient	Permanent	Owns	    INVALID, child entity will leak if parent removed	-
//	Transient	Permanent	MaybeOwns	INVALID, child entity will leak if parent removed	-
//	Transient	Permanent	ManyOwns	INVALID, child entity will leak if parent removed	-
//	Transient	Permanent	Ref	        A -- B	                                            colony references the body it's built upon
//	Transient	Permanent	MaybeRef	A -- Opt<B>	                                        ships can reference a system, but may not be in one
//	Transient	Permanent	ManyRef	    A -- [B]	                                        NEW
//	Transient	Transient	Owns	    A, B, C -> D	                                    shared component, only deleted with the owner
//	Transient	Transient	MaybeOwns	A -> Opt<B>	                                        optional or shared component, only deleted by the owner
//	Transient	Transient	ManyOwns	A -> [B]	                                        NEW
//	Transient	Transient	Ref	        MAYBE INVALID, must point at owner so that it can be deleted with it
//	Transient	Transient	MaybeRef	A -- Opt<B>                                         ship refers to its controller
//	Transient	Transient	ManyRef	    A -- [B]                                            NEW

// TODO examine cases of Owns Many
// Link         1:1
// MaybeLink    1:[0..1]
// ManyLink     1:[0..]     owner does not point to owned, rather all owned reference owner
// perhaps the valid case of T(a)-T(b)-Ref is actually a case of T(b)-T(a)-ManyOwn



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

    pub fn add_component(mut self, component: ComponentType) -> Self {
        self.components.push(component);
        self
    }

    pub fn add_default_component(mut self, component: ComponentType) -> Self {
        self.default_components.push(component);
        self
    }

    pub fn add_reference(&mut self, reference_to: &Arena, link_type: LinkType) {
        self.references.insert(reference_to.name.clone(), link_type);
    }

    pub fn add_ownership(&mut self, owned: &Arena, link_type: LinkType) {
        self.ownership.insert(owned.name.clone(), link_type);
    }

    pub fn get_allocator_field(&self) -> Field {
        Field {
            visibility: Visibility::Pub,
            name: self.name.clone().into(),
            field_type: self.allocator.get_type(self),
        }
    }

    pub fn get_state_field(&self) -> Field {
        let field_name: SnakeCase = self.name.clone().into();
        let field_type: Type = Type::new(self.name.as_str());

        Field {
            visibility: Default::default(),
            name: field_name,
            field_type
        }
    }

    pub fn get_struct(&self, world: &World) -> Struct {
        let link_fields = self.ownership.iter()
            .chain(self.references.iter())
            .map(|(link_to, link_type)| (world.get_arena(link_to), link_type))
            .filter_map(|(link_to, link_type)| self.get_link_component(link_to, link_type))
            .collect::<Vec<_>>();

        let fields = link_fields.iter()
            .chain(&self.components)
            .chain(&self.default_components)
            .map(ComponentType::get_arena_field)
            .collect();

        Struct {
            typ: self.get_arena_type(),
            visibility: Default::default(),
            derives: Derives::with_debug_default_clone(),
            fields,
        }
    }

    pub fn get_arena_type(&self) -> Type {
        self.name.clone().into()
    }

    pub fn get_data_row(&self) -> Struct {
        let mut name = self.name.to_string();
        name += "Row";
        let name = CamelCase::from_str(name.as_str()).unwrap();

        let fields = self.components.iter()
            .map(ComponentType::get_data_field)
            .collect();

        Struct::new(name.as_str())
            .with_derives(Derives::with_debug())
            .with_fields(fields)
    }

    fn get_link_component(&self, link_to: &Arena, link_type: &LinkType) -> Option<ComponentType> {
        let name: SnakeCase = link_to.name.clone().into();

        match link_type {
            LinkType::Required => ComponentType {
                name,
                data_type: link_to.get_id_type(),
                storage: Storage::Linear
            }.into(),
            LinkType::Optional => ComponentType {
                name,
                data_type: link_to.get_id_type(),
                storage: Storage::LinearOption
            }.into(),
            LinkType::Many => None,
        }
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
            .with_parameters(&format!("&mut self, id: &{}, row: {}", id, data_row));

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
            .add_line(CodeLine::new(0, "self.insert(&id, row);"))
            .add_line(CodeLine::new(0, "id"));

        Impl::from(&self.get_arena_type())
            .add_function(insert)
            .add_function(create)
    }

    pub fn get_id_type(&self) -> Type {
        self.allocator.get_id_type(&self)
    }

    pub fn owns(&self, arena: &Arena) -> bool {
        self.ownership.contains_key(&arena.name)
    }
}