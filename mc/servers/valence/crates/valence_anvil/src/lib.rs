#![doc = include_str!("../README.md")]

#[cfg(feature = "bevy_plugin")]
pub use bevy::*;

#[cfg(feature = "bevy_plugin")]
mod bevy;
mod directory;
#[cfg(feature = "parsing")]
pub mod parsing;
mod region;
#[cfg(feature = "parsing")]
pub mod snapshot;
mod types;

pub use directory::RegionFolder;
pub use types::{Compression, RawChunk, RegionError, WriteOptions};
