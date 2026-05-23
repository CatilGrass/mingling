//! Example Error Handling
//!
//! > This example demonstrates how to handle errors in Mingling, including custom error types and error rendering.
//!
//! Run:
//! ```bash
//! cargo run --manifest-path examples/example-exitcode/Cargo.toml --quiet -- hello Alice
//! cargo run --manifest-path examples/example-exitcode/Cargo.toml --quiet -- hello
//! ```
//!
//! Output:
//! ```plaintext
//! Hello, Alice
//! No name provided (with exit code 1)
//! ```

use mingling::{prelude::*, res::ExitCode, setup::ExitCodeSetup};

fn main() {
    let mut program = ThisProgram::new();

    // --------- IMPORTANT ---------
    // Register `ExitCodeSetup` for the program to enable exit codes
    program.with_setup(ExitCodeSetup::default());
    // --------- IMPORTANT ---------

    program.with_dispatcher(CMDHello);
    program.exec_and_exit();
}

dispatcher!("hello", CMDHello => EntryHello);

pack!(ErrorNoNameProvided = ());
pack!(ResultName = String);

#[chain]
fn handle_hello(args: EntryHello) -> Next {
    let Some(name) = args.inner.first().cloned() else {
        // If no name is provided, pass ErrorNoNameProvided
        return ErrorNoNameProvided::default().to_render();
    };

    // If the name is valid, pass ResultName
    ResultName::new(name).to_render()
}

#[renderer]
fn render_result_name(name: ResultName) {
    r_println!("Hello, {}", *name);
}

// Define renderer, render error message                      _____________ Inject exit code resource
//                                                           /
#[renderer] //                                               vvvvvvvvvvvvv
fn render_error_no_name_provided(_: ErrorNoNameProvided, ec: &mut ExitCode) {
    ec.exit_code = 1;

    // Prompt when no name is provided
    r_println!("No name provided (with exit code 1)");
}

gen_program!();
