use crate::{ProgramCollect, program::Program};

mod basic;
pub use basic::*;

pub trait ProgramSetup<C: ProgramCollect> {
    fn setup(&mut self, program: &mut Program<C>);
}

impl<C> Program<C>
where
    C: ProgramCollect,
{
    /// Load and execute init logic
    pub fn with_setup<S: ProgramSetup<C> + 'static>(&mut self, mut setup: S) -> S {
        S::setup(&mut setup, self);
        setup
    }
}
