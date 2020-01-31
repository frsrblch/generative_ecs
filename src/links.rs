use code_gen::CamelCase;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LinkType {
    Required,
    Optional,
    Many,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Link {
    pub from: CamelCase,
    pub to: CamelCase,
}