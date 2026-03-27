use crate::program::Program;

mod basic;
pub use basic::*;

pub trait ProgramSetup {
    fn setup(program: &mut Program);
}

impl Program {
    /// Load and execute init logic
    pub fn with_setup<S: ProgramSetup + 'static>(&mut self, _setup: S) {
        S::setup(self);
    }
}
