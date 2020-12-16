//! Logic for working with layers.
//!
//! Databases in terminus-store are stacks of layers. The first layer
//! in such a stack is a base layer, which contains an intial data
//! set. On top of that, each layer stores additions and removals.
mod base;
mod builder;
mod child;
mod delta;
mod id_map;
mod internal;
mod layer;
mod rollup;
mod simple_builder;

pub use base::*;
pub use child::*;
pub use delta::*;
pub use id_map::*;
pub use internal::*;
pub use layer::*;
pub use rollup::*;
pub use simple_builder::*;
