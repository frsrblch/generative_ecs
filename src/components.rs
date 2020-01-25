use code_gen::*;

#[derive(Debug)]
pub struct Component {
    pub name: SnakeCase,
    pub data_type: String,
    pub storage: Storage,
}

impl Component {
    pub fn get_field(&self) -> Field {
        Field {
            visibility: Visibility::Pub,
            name: self.name.clone(),
            field_type: self.storage.get_data_type(&self.data_type),
        }
    }
}

#[derive(Debug)]
pub enum Storage {
    Linear,
    LinearOption,
}

impl Storage {
    pub fn get_data_type(&self, data_type: &str) -> String {
        match self {
            Storage::Linear => format!("Component<Self, {}>", data_type),
            Storage::LinearOption => format!("Component<Self, Option<{}>>", data_type),
        }
    }
}

#[derive(Debug)]
pub struct StaticComponent {
    pub name: SnakeCase,
    pub data_type: String,
}

impl StaticComponent {
    pub fn get_field(&self) -> Field {
        Field {
            visibility: Visibility::Pub,
            name: self.name.clone(),
            field_type: self.data_type.clone(),
        }
    }
}
