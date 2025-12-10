//! Storage adapters for platform-specific persistence

mod adapter;
#[cfg(not(target_arch = "wasm32"))]
mod desktop;
#[cfg(target_arch = "wasm32")]
mod web;
pub mod encryption;
mod service;

pub use adapter::*;
#[cfg(not(target_arch = "wasm32"))]
pub use desktop::*;
#[cfg(target_arch = "wasm32")]
pub use web::*;
pub use encryption::*;
pub use service::*;
