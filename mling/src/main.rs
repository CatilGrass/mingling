use mingling::macros::gen_program;

pub mod cli;
pub mod display;
pub mod namespace_manager;
pub mod project_installer;
pub mod project_solver;

use crate::cli::*;

fn main() {
    cli_entry();
}

gen_program!();
