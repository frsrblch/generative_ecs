use crate::Arena;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use std::num::NonZeroU32;
use std::fmt::{Display, Formatter, Result};
use std::marker::PhantomData;
use code_gen::Type;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Allocator {
    Fixed,
    Generational,
}

impl Allocator {
    pub fn get_type(self, arena: &Arena) -> String {
        match self {
            Allocator::Fixed => format!("FixedAllocator<{}>", arena.name),
            Allocator::Generational => format!("GenAllocator<{}>", arena.name),
        }
    }

    pub fn get_id_type(self, arena: &Arena) -> Type {
        let s = match self {
            Allocator::Fixed => format!("Id<{}>", arena.name),
            Allocator::Generational => format!("GenId<{}>", arena.name),
        };

        Type::new(&s)
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Generation(NonZeroU32);

impl Display for Generation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0.get())
    }
}

impl Generation {
    pub fn next(self) -> Self {
        let next_gen = NonZeroU32::new(self.0.get() + 1).unwrap();
        Generation(next_gen)
    }

    pub fn value(self) -> u32 {
        self.0.get()
    }
}

impl Default for Generation {
    fn default() -> Self {
        Generation(NonZeroU32::new(1).unwrap())
    }
}

#[derive(Debug)]
pub struct Id<T> {
    pub (crate) index: usize,
    marker: PhantomData<T>,
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Id({})", self.index)
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self::new(self.index)
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index)
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl<T> Id<T> {
    pub (crate) fn new(index: usize) -> Self {
        Self {
            index,
            marker: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct GenId<T> {
    pub (crate) id: Id<T>,
    pub (crate) gen: Generation,
}

impl<T> Display for GenId<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "GenId({},{})", self.id.index, self.gen)
    }
}

impl<T> Clone for GenId<T> {
    fn clone(&self) -> Self {
        Self::new(self.id.index, self.gen)
    }
}

impl<T> Copy for GenId<T> {}

impl<T> GenId<T> {
    pub (crate) fn new(index: usize, gen: Generation) -> Self {
        Self {
            id: Id::new(index),
            gen,
        }
    }
}

impl<T> PartialEq for GenId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id) && self.gen.eq(&other.gen)
    }
}

impl<T> Eq for GenId<T> {}

impl<T> Hash for GenId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> PartialOrd for GenId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<T> Ord for GenId<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

#[derive(Debug)]
pub struct Valid<T> {
    pub (crate) id: GenId<T>,
}

impl<T> Valid<T> {
    pub (crate) fn new(id: GenId<T>) -> Self {
        Self {
            id,
        }
    }

    pub fn index(&self) -> usize {
        self.id.id.index
    }
}