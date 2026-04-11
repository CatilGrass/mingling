//! Mingling
//!
//! # Intro
//! `Mingling` is a Rust command-line framework. Its name comes from the Chinese Pinyin for "命令", which means "Command".
//!
//! # Use
//!
//! ```rust
//! use mingling::macros::{dispatcher, gen_program, r_println, renderer};
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut program = ThisProgram::new();
//!     program.with_dispatcher(HelloCommand);
//!
//!     // Execute
//!     program.exec().await;
//! }
//!
//! // Define command: "<bin> hello"
//! dispatcher!("hello", HelloCommand => HelloEntry);
//!
//! // Render HelloEntry
//! #[renderer]
//! fn render_hello_world(_prev: HelloEntry) {
//!     r_println!("Hello, World!")
//! }
//!
//! // Fallbacks
//! #[renderer]
//! fn fallback_dispatcher_not_found(prev: DispatcherNotFound) {
//!     r_println!("Dispatcher not found for command `{}`", prev.join(", "))
//! }
//!
//! #[renderer]
//! fn fallback_renderer_not_found(prev: RendererNotFound) {
//!     r_println!("Renderer not found `{}`", *prev)
//! }
//!
//! // Collect renderers and chains to generate ThisProgram
//! gen_program!();
//! ```
//!
// Output:
//!
//! ```text
//! > mycmd hello
//! Hello, World!
//! > mycmd hallo
//! Dispatcher not found for command `hallo`
//! ```
//!
//! # Features
//! - `parser` enables the `mingling::parser` module [More](./docs/parser/index.html)
//! - `general_renderer` adds support for serialized output formats such as JSON and YAML

// Re-export Core lib
pub use mingling::*;
pub use mingling_core as mingling;

/// `Mingling` argument parser
#[cfg(feature = "parser")]
pub mod parser;

/// Re-export from `mingling_macros`
#[allow(unused_imports)]
pub mod macros {
    /// Used to generate a struct implementing the `Chain` trait via a method
    pub use mingling_macros::chain;
    /// Used to generate completion entry
    #[cfg(feature = "comp")]
    pub use mingling_macros::completion;
    /// Used to create a dispatcher that routes to a `Chain`
    pub use mingling_macros::dispatcher;
    /// Used to create a dispatcher that routes to a `Renderer`
    pub use mingling_macros::dispatcher_render;
    /// Used to collect data and create a command-line context
    pub use mingling_macros::gen_program;
    /// Used to create a `Node` struct via a literal
    pub use mingling_macros::node;
    /// Used to create a wrapper type for use with `Chain` and `Renderer`
    pub use mingling_macros::pack;
    /// Internal macro for 'gen_program' used to finally generate the program
    pub use mingling_macros::program_final_gen;
    #[cfg(feature = "comp")]
    /// Internal macro for 'gen_program' used to finally generate the completion structure
    pub use mingling_macros::program_gen_completion;
    // Used to generate program setup
    pub use mingling_macros::program_setup;
    /// Used to print content within a `Renderer` context
    pub use mingling_macros::r_print;
    /// Used to print content with a newline within a `Renderer` context
    pub use mingling_macros::r_println;
    /// Used to generate a struct implementing the `Renderer` trait via a method
    pub use mingling_macros::renderer;
    #[cfg(feature = "comp")]
    /// Used to generate suggestions
    pub use mingling_macros::suggest;
}

/// derive macro Groupped
pub use mingling_macros::Groupped;

mod example_docs;

/// Example projects for `Mingling`, for learning how to use `Mingling`
pub mod _mingling_examples {
    pub use crate::example_docs::*;
}
