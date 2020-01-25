use crate::*;
use code_gen::*;
use std::convert::TryInto;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
pub struct World {
    pub arenas: Vec<Arena>,
    pub components: Vec<StaticComponent>,
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{}", self.get_world()).ok();
        writeln!(f, "{}", self.get_allocators()).ok();
        writeln!(f, "{}", self.get_state()).ok();
        Ok(())
    }
}

impl World {
    pub fn get_world(&self) -> Struct {
        Struct {
            name: "World".try_into().unwrap(),
            visibility: Visibility::Pub,
            fields: Fields::Standard(vec![
                Field {
                    visibility: Visibility::Pub,
                    name: "allocators".try_into().unwrap(),
                    field_type: "Allocators".to_string(),
                },
                Field {
                    visibility: Visibility::Pub,
                    name: "state".try_into().unwrap(),
                    field_type: "State".to_string()
                },
            ]),
        }
    }

    pub fn get_allocators(&self) -> Struct {
        let fields = self.arenas
            .iter()
            .map(|arena| Field {
                visibility: Visibility::Pub,
                name: arena.name.clone().into(),
                field_type: arena.allocator.get_type(&arena),
            })
            .collect();

        Struct {
            name: "Allocators".try_into().unwrap(),
            visibility: Visibility::Pub,
            fields: Fields::Standard(fields),
        }
    }

    pub fn get_state(&self) -> Struct {
        let mut fields: Vec<Field> =self.components
            .iter()
            .map(StaticComponent::get_field)
            .collect();

        fields.extend(
            self.arenas
                .iter()
                .map(Arena::get_state_field)
        );

        Struct {
            name: "State".try_into().unwrap(),
            visibility: Visibility::Pub,
            fields: Fields::Standard(fields),
        }
    }
}

#[test]
fn example() {
    let components = vec![
        StaticComponent {
            name: "time".try_into().unwrap(),
            data_type: "Time".to_string()
        }
    ];

    let arenas = vec![
        Arena {
            name: "System".try_into().unwrap(),
            allocator: Allocator::Fixed,
            components: vec![
                Component {
                    name: "name".try_into().unwrap(),
                    data_type: "String".to_string(),
                    storage: Storage::Linear
                },
                Component {
                    name: "position".try_into().unwrap(),
                    data_type: "Position".to_string(),
                    storage: Storage::Linear
                },
            ]
        },
        Arena {
            name: "Body".try_into().unwrap(),
            allocator: Allocator::Fixed,
            components: vec![
                Component {
                    name: "name".try_into().unwrap(),
                    data_type: "String".to_string(),
                    storage: Storage::LinearOption,
                },
                Component {
                    name: "position".try_into().unwrap(),
                    data_type: "Position".to_string(),
                    storage: Storage::Linear
                },
            ]
        },
    ];

    let world = World {
        arenas,
        components,
    };

    println!("{}", world);

    assert!(false);
}