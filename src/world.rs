use crate::*;
use code_gen::*;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
pub struct World {
    pub name: CamelCase,
    pub arenas: Vec<Arena>,
    pub components: Vec<StaticComponent>,
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
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
    pub fn new(name: &str) -> Self {
        World {
            name: name.parse().unwrap(),
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
            .with_return(&return_type)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
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

//        let atmosphere = Arena::fixed("Atmosphere")
//            .add_component(Component::dense("breathability", "bool"))
//            .add_component(Component::dense_from_type("GreenhouseRatio"));

        let world = World::new("Game")
            .add_static_component(StaticComponent::from_type("Time"))
            .add_static_component(StaticComponent::from_type("Starfield"))
            .add_arena(system)
            .add_arena(body)
            .add_arena(surface)
//            .add_arena(atmosphere)
            ;

        println!("{}", world);

        assert!(false);
    }
}