use code_gen::*;
use std::convert::TryInto;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Component {
    pub name: SnakeCase,
    pub data_type: String,
    pub storage: Storage,
}

impl Component {
    pub fn dense(name: &str, data_type: &str) -> Self {
        Self {
            name: name.try_into().unwrap(),
            data_type: data_type.to_string(),
            storage: Storage::Linear,
        }
    }

    pub fn dense_from_type(data_type: impl TryInto<CamelCase, Error=impl Debug>) -> Self {
        let data_type = data_type.try_into().unwrap();
        Self {
            name: data_type.clone().into(),
            data_type: data_type.to_string(),
            storage: Storage::Linear,
        }
    }

    pub fn sparse(name: &str, data_type: &str) -> Self {
        Self {
            name: name.try_into().unwrap(),
            data_type: data_type.to_string(),
            storage: Storage::LinearOption,
        }
    }

    pub fn sparse_from_type(data_type: impl TryInto<CamelCase, Error=impl Debug>) -> Self {
        let data_type = data_type.try_into().unwrap();
        Self {
            name: data_type.clone().into(),
            data_type: data_type.to_string(),
            storage: Storage::LinearOption,
        }
    }

    pub fn get_arena_field(&self) -> Field {
        Field {
            visibility: Visibility::Pub,
            name: self.name.clone(),
            field_type: self.storage.get_component_data_type(&self.data_type),
        }
    }

    pub fn get_data_field(&self) -> Field {
        Field::new(self.name.clone(), &self.storage.get_row_data_type(self.data_type.as_str()))
    }
}

#[derive(Debug)]
pub enum Storage {
    Linear,
    LinearOption,
}

impl Storage {
    pub fn get_component_data_type(&self, data_type: &str) -> String {
        match self {
            Storage::Linear => format!("Component<Self, {}>", data_type),
            Storage::LinearOption => format!("Component<Self, Option<{}>>", data_type),
        }
    }

    pub fn get_row_data_type(&self, data_type: &str) -> String {
        match self {
            Storage::Linear => data_type.to_string(),
            Storage::LinearOption => format!("Option<{}>", data_type),
        }
    }
}

#[derive(Debug)]
pub struct StaticComponent {
    pub name: SnakeCase,
    pub data_type: String,
}

impl StaticComponent {
    pub fn new(name: &str, data_type: &str) -> Self {
        Self {
            name: name.try_into().unwrap(),
            data_type: data_type.to_string(),
        }
    }

    pub fn from_type<E: Debug>(data_type: impl TryInto<CamelCase,Error=E>) -> Self {
        let data_type = data_type.try_into().unwrap();
        Self {
            name: data_type.clone().into(),
            data_type: data_type.to_string(),
        }
    }

    pub fn get_field(&self) -> Field {
        Field {
            visibility: Visibility::Pub,
            name: self.name.clone(),
            field_type: self.data_type.clone(),
        }
    }
}
