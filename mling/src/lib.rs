#![allow(unused_imports)]

use colored::Colorize;
use mingling::{
    macros::{chain, gen_program, pack, r_println, renderer},
    res::ResExitCode,
};

pub mod cli;
pub use cli::*;

mod cargo_style;
pub use cargo_style::*;
pub mod display;
pub mod res;

mod pkg_mgr;
pub use pkg_mgr::*;

mod proj_mgr;
pub use proj_mgr::*;

use crate::display::markdown;

pack!(ResultMlingHelp = ());
pack!(ResultUnknownCommand = String);

#[chain]
fn handle_error_dispatcher_not_found(err: ErrorDispatcherNotFound) -> Next {
    if err.is_empty() {
        ResultMlingHelp::default().to_render()
    } else {
        ResultUnknownCommand::new(err.join(" ")).to_render()
    }
}

#[renderer]
fn render_mling_help(_prev: ResultMlingHelp, ec: &mut ResExitCode) {
    r_println!("{}", markdown(include_str!("helps/mling_help.txt")));
    ec.exit_code = 0;
}

#[renderer]
fn render_unknown_command(prev: ResultUnknownCommand, ec: &mut ResExitCode) {
    r_println!(
        "{}",
        eformat_cargo!("no such command: `{}`", prev.bright_yellow().bold())
    );
    r_println!();
    r_println!(
        "{}",
        hformat_cargo!("view all commands with `cargo help mling`")
    );
    ec.exit_code = 101;
}

gen_program!();
