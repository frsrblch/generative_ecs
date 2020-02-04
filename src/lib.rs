mod world;
mod arenas;
mod components;
mod links;
mod traits;
mod ids;
mod allocators;
mod compound_entity;

pub use world::*;
pub use traits::*;
pub use arenas::*;
pub use components::*;
pub use links::*;
pub use ids::*;
pub use allocators::*; // TODO setup prelude
pub use prelude::*;
pub use compound_entity::*;

pub mod prelude {
    pub use crate::allocators::{FixedAllocator, GenAllocator, Component};
    pub use crate::ids::{Id, GenId, Valid};
    pub use crate::traits::*;
}