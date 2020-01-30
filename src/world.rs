use crate::*;
use code_gen::*;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
pub struct World {
    pub arenas: Vec<Arena>,
    pub components: Vec<StaticComponent>,
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.validate();

        writeln!(f, "{}", self.get_world()).ok();
        writeln!(f, "{}", self.impl_world()).ok();
        writeln!(f, "{}", self.get_allocators()).ok();
        writeln!(f, "{}", self.get_state()).ok();

        for arena in self.arenas.iter() {
            writeln!(f, "{}", arena.get_struct()).ok();
            writeln!(f, "{}", arena.get_impl()).ok();
            writeln!(f, "{}", arena.get_data_row()).ok();
        }

        Ok(())
    }
}

impl World {
    pub fn new() -> Self {
        World {
            arenas: vec![],
            components: vec![]
        }
    }

    pub fn add_arena(mut self, arena: Arena) -> Self {
        self.arenas.push(arena);
        self
    }

    pub fn add_static_component(mut self, component: StaticComponent) -> Self {
        self.components.push(component);
        self
    }

    pub fn get_world(&self) -> Struct {
        Struct::new("World")
            .with_derives(Derives::with_debug_default_clone())
            .add_field(Field::from_type(Type::new("Allocators")))
            .add_field(Field::from_type(Type::new("State")))
    }

    pub fn impl_world(&self) -> Impl {
        Impl::new(self.get_world().typ)
            .add_function(self.get_split())
    }

    fn get_split(&self) -> Function {
        let fields = self.get_world().fields;

        let return_type: Vec<String> = fields.iter().cloned().map(|f| f.field_type).collect();
        let return_fields: Vec<SnakeCase> = fields.iter().cloned().map(|f| f.name).collect();

        let return_type = StrConcat {
            iter: return_type,
            left_bound: "(",
            right_bound: ")",
            item_prepend: "&mut ",
            item_append: "",
            join: ", "
        }.to_string();

        let code = StrConcat {
            iter: return_fields,
            left_bound: "(",
            right_bound: ")",
            item_prepend: "&mut self.",
            item_append: "",
            join: ", "
        }.to_string();

        Function::new("split")
            .with_parameters("&mut self")
            .with_return(return_type)
            .add_line(CodeLine::new(0, &code))
    }

    pub fn get_allocators(&self) -> Struct {
        let fields = self.arenas.iter()
            .map(Arena::get_allocator_field)
            .collect();

        Struct::new("Allocators")
            .with_derives(Derives::with_debug_default_clone())
            .with_fields(fields)
    }

    pub fn get_state(&self) -> Struct {
        let static_fields = self.components.iter().map(StaticComponent::get_field);
        let arena_fields = self.arenas.iter().map(Arena::get_state_field);

        let fields = static_fields
            .chain(arena_fields)
            .collect();

        Struct::new("State")
            .with_derives(Derives::with_debug_default_clone())
            .with_fields(fields)
    }

    fn validate(&self) {
        assert!(self.no_transient_owns_permanent());
        assert!(self.no_permanent_has_mandatory_link_to_transient());
    }

    fn no_transient_owns_permanent(&self) -> bool {
        let owns_permanent = |arena: &Arena| {
            arena.ownership.keys()
                .map(|k| self.get_arena(k))
                .any(|owned| owned.allocator == Allocator::Fixed)
        };

        !self.transient_entities().any(owns_permanent)
    }

    fn get_arena(&self, name: &CamelCase) -> &Arena {
        self.arenas.iter()
            .find(|a| a.name.eq(name))
            .expect(&format!("Expected arena not found in World: {}", name))
    }

    fn no_permanent_has_mandatory_link_to_transient(&self) -> bool {
        let mandatory_link_to_temporary = |arena: &Arena| {
            arena.ownership.iter()
                .chain(arena.references.iter())
                .map(|(name, link)| (self.get_arena(name), link))
                .any(|(owned, link)| {
                    owned.allocator == Allocator::Generational && *link == LinkType::Required
                })
        };

        !self.permanent_entities().any(mandatory_link_to_temporary)
    }

    fn permanent_entities(&self) -> impl Iterator<Item=&Arena> {
        self.arenas.iter()
            .filter(|arena| arena.allocator == Allocator::Fixed)
    }

    fn transient_entities(&self) -> impl Iterator<Item=&Arena> {
        self.arenas.iter()
            .filter(|arena| arena.allocator == Allocator::Generational)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_space_example() -> World {
        let system = Arena::fixed("System")
            .add_component(Component::dense("name", "String"))
            .add_component(Component::dense_from_type("Position"))
            .add_component(Component::dense("radius", "Length"))
            .add_component(Component::dense_from_type("Temperature"))
            .add_default_component(Component::dense_from_type("Camera"));

        let body = Arena::fixed("Body")
            .add_component(Component::sparse("name", "String"))
            .add_component(Component::dense("parameters", "OrbitParameters"))
            .add_component(Component::sparse("parent", "Id<Body>"))
            .add_default_component(Component::dense_from_type("Position"))
            .add_default_component(Component::dense("relative_pos", "Position"));

        let surface = Arena::fixed("Surface")
            .add_component(Component::dense_from_type("Area"))
            .add_default_component(Component::dense_from_type("Temperature"));

        let atmosphere = Arena::fixed("Atmosphere")
            .add_component(Component::dense("breathability", "bool"))
            .add_component(Component::dense_from_type("GreenhouseRatio"));

        World::new()
            .add_static_component(StaticComponent::from_type("Time"))
            .add_static_component(StaticComponent::from_type("Starfield"))
            .add_arena(system)
            .add_arena(body)
            .add_arena(surface)
            .add_arena(atmosphere)
    }

    //	Transient	Permanent	Owns	    INVALID, child entity will leak if parent removed	-
    //	Transient	Permanent	Maybe Owns	INVALID, child entity will leak if parent removed	-
    #[test]
    #[should_panic]
    fn invalid_temporary_owning_permanent() {
        let perm = Arena::fixed("Perm");

        let temp = Arena::generational("Temp")
            .add_ownership(&perm, LinkType::Required);

        let invalid = World::new()
            .add_arena(perm)
            .add_arena(temp);

        invalid.validate();
    }

    //	Permanent	Transient	Owns	    INVALID, no reason for child to be temp if it cannot unlink
    #[test]
    #[should_panic]
    fn invalid_permanent_cannot_mandatory_own_temporary() {
        let temp = Arena::generational("Temp");

        let perm = Arena::fixed("Perm")
            .add_ownership(&temp, LinkType::Required);

        let invalid = World::new()
            .add_arena(perm)
            .add_arena(temp);

        invalid.validate();
    }

    //	Permanent	Transient	Ref	        INVALID, cannot be unlinked if child removed	    -
    #[test]
    #[should_panic]
    fn invalid_permanent_cannot_mandatory_refer_to_temporary() {
        let temp = Arena::generational("Temp");

        let perm = Arena::fixed("Perm")
            .add_reference(&temp, LinkType::Required);

        let invalid = World::new()
            .add_arena(perm)
            .add_arena(temp);

        invalid.validate();
    }
}