#![allow(unused_imports)]

use mingling::{
    macros::{gen_program, r_println, renderer},
    res::ResExitCode,
};

mod cargo_style;
pub use cargo_style::*;
pub mod display;
pub mod res;

mod proj_mgr;
use proj_mgr::*;

use crate::display::markdown;

#[renderer]
pub fn render_error_dispatch_not_found(err: ErrorDispatcherNotFound, ec: &mut ResExitCode) {
    if err.len() < 1 {
        r_println!("{}", markdown(include_str!("helps/mling_help.txt")));
        ec.exit_code = 0;
    } else {
        let s = eformat_cargo!("Cannot find subcommand \"{}\"", err.join(" "));
        r_println!("{}", s);
        ec.exit_code = 1;
    }
}

gen_program!();
