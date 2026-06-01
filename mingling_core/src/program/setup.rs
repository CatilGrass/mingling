use crate::{ProgramCollect, program::Program};

pub trait ProgramSetup<C>
where
    C: ProgramCollect<Enum = C>,
{
    fn setup(self, program: &mut Program<C>);
}

impl<C> Program<C>
where
    C: ProgramCollect<Enum = C>,
{
    /// Load and execute init logic
    pub fn with_setup<S: ProgramSetup<C> + 'static>(&mut self, setup: S) {
        S::setup(setup, self);
    }
}
