#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LinkType {
    Required,
    Optional,
    Many,
}
