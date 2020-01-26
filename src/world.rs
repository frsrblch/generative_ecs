use crate::*;
use code_gen::*;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Default)]
pub struct World {
    pub arenas: Vec<Arena>,
    pub components: Vec<StaticComponent>,
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//        writeln!(f, "{}", self.get_world()).ok();
        writeln!(f, "{}", self.impl_world()).ok();
//        writeln!(f, "{}", self.get_allocators()).ok();
//        writeln!(f, "{}", self.get_state()).ok();
//
//        for arena in self.arenas.iter() {
//            writeln!(f, "{}", arena.get_struct()).ok();
//            writeln!(f, "{}", arena.get_data_row()).ok();
//        }

        Ok(())
    }
}

impl World {
    pub fn new() -> Self { Default::default() }

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
            .add_field(Field::from_type("Allocators"))
            .add_field(Field::from_type("State"))
    }

    pub fn impl_world(&self) -> Impl {
        let world = self.get_world();

        let params = "&mut self";

        let mut return_type = String::from("(");
        let mut code = String::from("(");
        for (i, field) in world.fields.iter().enumerate() {
            if i != 0 {
                return_type.push_str(", ");
                code.push_str(", ");
            }

            return_type.push_str(&format!("&mut {}", field.field_type));
            code.push_str(&format!("&mut self.{}", field.name));
        }
        return_type.push(')');
        code.push(')');

        Impl::new(&world)
            .add_function(Function::new("split")
                .with_parameters(&params)
                .with_return(&return_type)
                .add_line(CodeLine::new(0, &code))
            )
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
        .add_component(Component::dense_from_type("Position"));

    let orbit = Arena::fixed("Orbit")
        .add_component(Component::dense("parameters", "OrbitParameters"))
        .add_component(Component::sparse("parent", "Id<Orbit>"))
        .add_default_component(Component::dense("relative_pos", "Position"));

    let world = World::new()
        .add_static_component(StaticComponent::from_type("Time"))
        .add_static_component(StaticComponent::from_type("Starfield"))
        .add_arena(system)
        .add_arena(body)
        .add_arena(orbit);

    println!("{}", world);

    assert!(false);
}