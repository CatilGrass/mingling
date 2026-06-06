#![allow(unused_imports)]

use mingling::macros::gen_program;

pub mod cargo_style;
pub mod display;
pub mod res;

mod proj_mgr;
use proj_mgr::*;

gen_program!();
