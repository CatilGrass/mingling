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
//! ```
//! > mycmd hello
//! Hello, World!
//! > mycmd hallo
//! Dispatcher not found for command `hallo`
//! ```

// Re-Export Core lib
pub use mingling::*;
pub use mingling_core as mingling;

#[cfg(feature = "parser")]
pub mod parser;

#[allow(unused_imports)]
pub mod macros {
    pub use mingling_macros::chain;
    pub use mingling_macros::dispatcher;
    pub use mingling_macros::dispatcher_render;
    pub use mingling_macros::gen_program;
    pub use mingling_macros::node;
    pub use mingling_macros::pack;
    pub use mingling_macros::r_print;
    pub use mingling_macros::r_println;
    pub use mingling_macros::renderer;
}
