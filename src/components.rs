use code_gen::*;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug)]
pub struct Component {
    pub name: SnakeCase,
    pub data_type: Type,
    pub storage: Storage,
}

impl Component {
    pub fn dense(name: &str, data_type: &str) -> Self {
        Self {
            name: name.parse().unwrap(),
            data_type: data_type.parse().unwrap(),
            storage: Storage::Linear,
        }
    }

    pub fn dense_from_type(data_type: &str) -> Self {
        let data_type: CamelCase = data_type.parse().unwrap();
        Self {
            name: data_type.clone().into(),
            data_type: data_type.into(),
            storage: Storage::Linear,
        }
    }

    pub fn sparse(name: &str, data_type: &str) -> Self {
        Self {
            name: name.parse().unwrap(),
            data_type: Type::from_str(data_type).unwrap(),
            storage: Storage::LinearOption,
        }
    }

    pub fn sparse_from_type(data_type: &str) -> Self {
        let data_type: CamelCase = data_type.parse().unwrap();
        Self {
            name: data_type.clone().into(),
            data_type: data_type.into(),
            storage: Storage::LinearOption,
        }
    }

    pub fn get_arena_field(&self) -> Field {
        Field {
            visibility: Visibility::Pub,
            name: self.name.clone(),
            field_type: self.storage.get_component_data_type(self.data_type.clone()),
        }
    }

    pub fn get_data_field(&self) -> Field {
        Field::new(self.name.clone(), self.storage.get_row_data_type(&self.data_type.to_string()))
    }
}

#[derive(Debug)]
pub enum Storage {
    Linear,
    LinearOption,
}

impl Storage {
    pub fn get_component_data_type(&self, data_type: Type) -> String {
        match self {
            Storage::Linear => format!("Component<Self, {}>", data_type),
            Storage::LinearOption => format!("Component<Self, Option<{}>>", data_type),
        }
    }

    pub fn get_row_data_type(&self, data_type: &str) -> Type {
        let s = match self {
            Storage::Linear => data_type.to_string(),
            Storage::LinearOption => format!("Option<{}>", data_type),
        };
        Type::new(s.as_str())
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
            name: name.parse().unwrap(),
            data_type: data_type.to_string(),
        }
    }

    pub fn from_type(data_type: &str) -> Self {
        let data_type: CamelCase = data_type.parse().unwrap();
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
