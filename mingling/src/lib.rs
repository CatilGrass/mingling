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
//!     let mut program = DefaultProgram::new();
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
//! // Collect renderers and chains to generate DefaultProgram
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
    /// Used to print content within a `Renderer` context
    pub use mingling_macros::r_print;
    /// Used to print content with a newline within a `Renderer` context
    pub use mingling_macros::r_println;
    /// Used to generate a struct implementing the `Renderer` trait via a method
    pub use mingling_macros::renderer;
}
