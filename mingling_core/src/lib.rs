//! Mingling Core
//!
//! # Intro
//! This crate is the core implementation of `mingling`, containing the complete logic for command dispatching, execution, and rendering.
//!
//! # Note
//! It is not recommended to use [mingling_core](https://crates.io/crates/mingling_core) directly, as this will lose the code generation functionality of [mingling_macros](https://crates.io/crates/mingling_macros).
//!
//! Recommended to import [mingling](https://crates.io/crates/mingling) to use its features.

mod any;
mod asset;
mod markers;
mod program;
mod renderer;

#[cfg(feature = "general_renderer")]
pub use crate::renderer::general::GeneralRenderer;

pub use crate::any::group::*;
pub use crate::any::*;

pub use crate::asset::chain::*;
#[cfg(feature = "comp")]
pub use crate::asset::comp::*;
pub use crate::asset::dispatcher::*;
pub use crate::asset::node::*;
pub use crate::asset::renderer::*;

/// All error types of `Mingling`
pub mod error {
    pub use crate::asset::chain::error::*;
    pub use crate::exec::error::*;
    #[cfg(feature = "general_renderer")]
    pub use crate::renderer::general::error::*;
}

pub use crate::program::*;

pub use crate::renderer::render_result::*;

/// All marker types of `Mingling` that serve no practical purpose
pub mod marker {
    pub use crate::markers::next_process::*;
    pub use crate::markers::this_program::*;
}

/// `Mingling`'s Program initialization system
pub mod setup {
    pub use crate::program::setup::*;
}
