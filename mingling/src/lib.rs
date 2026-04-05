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
    // Used to generate program setup
    pub use mingling_macros::program_setup;
    /// Used to print content within a `Renderer` context
    pub use mingling_macros::r_print;
    /// Used to print content with a newline within a `Renderer` context
    pub use mingling_macros::r_println;
    /// Used to generate a struct implementing the `Renderer` trait via a method
    pub use mingling_macros::renderer;
}

/// derive macro Groupped
pub use mingling_macros::Groupped;

pub mod docs {
    pub mod basic {
        //! # Basic Usage
        //!
        //! This module demonstrates basic usage of Mingling with a simple "hello" command.
        //!
        //! ## Example
        //!
        //! ```rust
        //! use mingling::{
        //!     macros::{chain, dispatcher, gen_program, pack, r_println, renderer},
        //!     marker::NextProcess,
        //! };
        //!
        //! // Define dispatcher `HelloCommand`, directing subcommand "hello" to `HelloEntry`
        //! dispatcher!("hello", HelloCommand => HelloEntry);
        //!
        //! #[tokio::main]
        //! async fn main() {
        //!     // Create program
        //!     let mut program = DefaultProgram::new();
        //!
        //!     // Add dispatcher `HelloCommand`
        //!     program.with_dispatcher(HelloCommand);
        //!
        //!     // Run program
        //!     program.exec().await;
        //! }
        //!
        //! // Register wrapper type `Hello`, setting inner to `String`
        //! pack!(Hello = String);
        //!
        //! // Register chain to `DefaultProgram`, handling logic from `HelloEntry`
        //! #[chain]
        //! async fn parse_name(prev: HelloEntry) -> NextProcess {
        //!     // Extract string from `HelloEntry` as argument
        //!     let name = prev.get(0).cloned().unwrap_or_else(|| "World".to_string());
        //!
        //!     // Build `Hello` type and route to renderer
        //!     Hello::new(name).to_render()
        //! }
        //!
        //! // Register renderer to `DefaultProgram`, handling rendering of `Hello`
        //! #[renderer]
        //! fn render_hello_who(prev: Hello) {
        //!     // Print message
        //!     r_println!("Hello, {}!", *prev);
        //!
        //!     // Program ends here
        //! }
        //!
        //! // Generate program, default is `DefaultProgram`
        //! gen_program!();
        //! ```
        //!
        //! ## Output
        //!
        //! ```text
        //! > mycmd hello
        //! Hello, World!
        //! > mycmd hello Alice
        //! Hello, Alice!
        //! ```
    }

    pub mod parser {
        //! # Feature `parser` Usage
        //!
        //! This module demonstrates advanced usage of Mingling with the `Picker` utility for argument parsing.
        //!
        //! ## Example
        //!
        //! ```rust
        //! use mingling::{
        //!     AnyOutput,
        //!     macros::{chain, dispatcher, gen_program, pack, r_println, renderer},
        //!     marker::NextProcess,
        //!     parser::Picker,
        //! };
        //!
        //! // Define dispatcher `RepeatCommand`, directing subcommand "repeat" to `RepeatEntry`
        //! dispatcher!("repeat", RepeatCommand => RepeatEntry);
        //!
        //! #[tokio::main]
        //! async fn main() {
        //!     // Create program
        //!     let mut program = DefaultProgram::new();
        //!
        //!     // Add dispatcher `RepeatCommand`
        //!     program.with_dispatcher(RepeatCommand);
        //!
        //!     // Run program
        //!     program.exec().await;
        //! }
        //!
        //! // Register wrapper type `RepeatArgument`, setting inner to `(i32, String)`
        //! pack!(RepeatArgument = (i32, String));
        //!
        //! // Register error type
        //! pack!(ErrorContentRequired = ());
        //!
        //! // Register chain to `DefaultProgram`, handling logic for `RepeatEntry`
        //! #[chain]
        //! async fn parse_repeat_args(prev: RepeatEntry) -> NextProcess {
        //!     let picker = Picker::new(prev.inner); // Create Picker from user arguments
        //!     let picked = picker
        //!         .pick_or::<i32>("--time", 1) // Extract argument `--time`
        //!         .after(|n| n.clamp(1, 20)) // Clamp extracted number between 1 - 20
        //!         // Extract first remaining argument as content, route to type `ErrorContentRequired` if not found
        //!         .pick_or_route((), AnyOutput::new(ErrorContentRequired::default()))
        //!         .unpack(); // Unpack
        //!
        //!     match picked {
        //!         Ok(args) => {
        //!             // Build `RepeatArgument` type and route to renderer
        //!             RepeatArgument::new(args).to_render()
        //!         }
        //!         Err(e) => {
        //!             // Extraction failed, route to error type
        //!             e.route_renderer()
        //!         }
        //!     }
        //! }
        //!
        //! // Render `RepeatArgument`
        //! #[renderer]
        //! fn render_repeat(prev: RepeatArgument) {
        //!     let (times, content) = prev.inner;
        //!     for _ in 0..times {
        //!         r_println!("{}", content);
        //!     }
        //! }
        //!
        //! // Render `ErrorContentRequired`
        //! #[renderer]
        //! fn render_error_content_required(_prev: ErrorContentRequired) {
        //!     r_println!("Error: content is required");
        //! }
        //!
        //! // Generate program, default is `DefaultProgram`
        //! gen_program!();
        //! ```
        //!
        //! ## Output
        //!
        //! ```text
        //! > mycmd repeat --time 3 Hello
        //! Hello
        //! Hello
        //! Hello
        //! > mycmd repeat --time 25 Hello
        //! Hello
        //! Hello
        //! Hello
        //! ... (repeated 20 times, clamped from 25)
        //! > mycmd repeat --time 3
        //! Error: content is required
        //! ```
    }
}
