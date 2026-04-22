use crate::{ProgramCollect, program::Program};

mod basic;
pub use basic::*;

#[cfg(feature = "general_renderer")]
mod general_renderer;
#[cfg(feature = "general_renderer")]
pub use general_renderer::*;

pub trait ProgramSetup<C, G>
where
    C: ProgramCollect,
{
    fn setup(&mut self, program: &mut Program<C, G>);
}

impl<C, G> Program<C, G>
where
    C: ProgramCollect,
{
    /// Load and execute init logic
    pub fn with_setup<S: ProgramSetup<C, G> + 'static>(&mut self, mut setup: S) -> S {
        S::setup(&mut setup, self);
        setup
    }
}
