//! Deterministic compatibility fixture cores shared by Valence examples.
//!
//! These modules intentionally avoid Bevy ECS access, filesystem I/O, process
//! state, environment reads, and logging. Example binaries remain the imperative
//! shells that translate Valence events/resources into these pure inputs and
//! apply the returned decisions or milestone text.

pub mod ctf;
pub mod survival;
