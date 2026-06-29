#![recursion_limit = "600"]
#![allow(clippy::identity_op)]
#![allow(clippy::collapsible_if)]

extern crate steven_shared as shared;

mod id_map;
pub mod material;
mod runtime;

pub use self::id_map::VanillaIDMap;
pub use self::material::Material;
pub use self::runtime::generated;
pub use self::runtime::generated::gen_id_map;
pub use self::runtime::generated::Block;
pub use self::runtime::generated::Block::*;
pub use self::runtime::*;

#[cfg(test)]
mod tests;
