//! `Mingling` Example - Exit Code
//!
//! This example demonstrates how to modify the program's exit code using `ExitCodeSetup`.
//! By default, the program exits with code 0. This example shows:
//! 1. Using `dispatcher!` to define an error command,
//! 2. Using `chain!` to handle errors and set a custom exit code via `ProgramExitCode`,
//! 3. Using `renderer!` to print an error message.
//!
//! # How to Run
//! ```bash
//! cargo run --manifest-path ./examples/example-exit-code/Cargo.toml -- error
//! ```

use mingling::prelude::*;
use mingling::{res::ExitCode, setup::ExitCodeSetup};

fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(ErrorCommand);
    program.with_setup(ExitCodeSetup::<ThisProgram>::default());
    program.exec_and_exit();
}

dispatcher!("error", ErrorCommand => ErrorEntry);
pack!(ResultError = ());

#[chain]
fn handle_error_entry(_prev: ErrorEntry, ec: &mut ExitCode) -> Next {
    ec.exit_code = 1;
    return ResultError::default();
}

#[renderer]
fn render_error(_prev: ResultError, ec: &ExitCode) {
    r_println!("Exit with exit code: {}", ec.exit_code);
}

gen_program!();
