use std::fmt::Display;

use crate::{ProgramCollect, program::Program};

mod basic;
pub use basic::*;

pub trait ProgramSetup<C, G>
where
    C: ProgramCollect,
    G: Display,
{
    fn setup(&mut self, program: &mut Program<C, G>);
}

impl<C, G> Program<C, G>
where
    C: ProgramCollect,
    G: Display,
{
    /// Load and execute init logic
    pub fn with_setup<S: ProgramSetup<C, G> + 'static>(&mut self, mut setup: S) -> S {
        S::setup(&mut setup, self);
        setup
    }
}
