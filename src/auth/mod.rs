//! Authentication handlers for Docker Registry API

mod challenge;
mod handler;

pub use challenge::*;
pub use handler::*;
