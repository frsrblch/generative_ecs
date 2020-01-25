use crate::Arena;

#[derive(Debug)]
pub enum Allocator {
    Fixed,
    Generational,
}

impl Allocator {
    pub fn get_type(&self, arena: &Arena) -> String {
        match self {
            Allocator::Fixed => format!("FixedAllocator<{}>", arena.name),
            Allocator::Generational => format!("GenAllocator<{}>", arena.name),
        }
    }
}