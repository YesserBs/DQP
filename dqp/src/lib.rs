//! DQP 1.0: strict compilation, deterministic execution and replay.
pub mod compiler;
pub mod diagnostic;
pub mod engine;
pub mod model;
pub mod source;
pub use compiler::{compile, load, Artifact};
pub use engine::{Engine, Report};
