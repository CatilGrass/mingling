//! Example Error Handling
//!
//! > This example demonstrates how to handle errors in Mingling, including custom error types and error rendering.
//!
//! Run:
//! ```bash
//! cargo run --manifest-path examples/example-error-handling/Cargo.toml --quiet -- hallo
//! cargo run --manifest-path examples/example-error-handling/Cargo.toml --quiet -- hello
//! cargo run --manifest-path examples/example-error-handling/Cargo.toml --quiet -- hello Alice
//! cargo run --manifest-path examples/example-error-handling/Cargo.toml --quiet -- hello MyBestFriendAlice
//! cargo run --manifest-path examples/example-error-handling/Cargo.toml --quiet -- hello Peter
//! ```
//!
//! Output:
//! ```plaintext
//! Command not found: "hallo"
//! No name provided
//! Name not available
//! Name too long: 17 > 10
//! Hello, Peter
//! ```

use mingling::prelude::*;

// In Mingling, instead of using ? to propagate errors upward,
// errors are treated as branches that continue execution.

dispatcher!("hello", CMDHello => EntryHello);

// Define error types
pack!(ErrorNoNameProvided = ());
pack!(ErrorNameTooLong = u16);
pack!(ErrorNameNotAvailable = ());

// Define success type
pack!(ResultName = String);

// Pre-registered names
static VEC_REGISTERED_NAMES: &[&str] = &["Alice", "Bob", "Charlie", "David", "Eve"];

#[chain]
fn handle_hello(args: EntryHello) -> Next {
    let Some(name) = args.inner.first().cloned() else {
        // If no name is provided, pass ErrorNoNameProvided
        return ErrorNoNameProvided::default().to_render();
    };

    if name.len() > 10 {
        // If the name is too long, pass ErrorNameTooLong
        return ErrorNameTooLong::new(name.len() as u16).to_render();
    }

    if VEC_REGISTERED_NAMES.contains(&name.as_str()) {
        // If the name already exists, pass ErrorNameNotAvailable
        return ErrorNameNotAvailable::default().to_render();
    }

    // If the name is valid, pass ResultName
    ResultName::new(name).to_render()
}

#[renderer]
fn render_result_name(name: ResultName) {
    r_println!("Hello, {}", *name);
}

#[renderer]
fn render_error_no_name_provided(_: ErrorNoNameProvided) {
    // Prompt when no name is provided
    r_println!("No name provided");
}

#[renderer]
fn render_error_name_not_available(_: ErrorNameNotAvailable) {
    // Prompt when name is already taken
    r_println!("Name not available");
}

#[renderer]
fn render_error_name_too_long(len: ErrorNameTooLong) {
    // Prompt when name is too long, showing actual length
    r_println!("Name too long: {} > 10", *len);
}

#[renderer]
fn render_dispatcher_not_found(err: DispatcherNotFound) {
    // Prompt when command is not found, showing the input command
    r_println!("Command not found: \"{}\"", err.inner.join(" "));
}

gen_program!();

fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(CMDHello);
    program.exec_and_exit();
}
