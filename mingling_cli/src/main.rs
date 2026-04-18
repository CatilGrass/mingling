use mingling::{macros::gen_program, setup::BasicProgramSetup};

mod dispatcher_mgr;
pub use crate::dispatcher_mgr::*;

fn main() {
    let mut program = ThisProgram::new();
    program.with_setup(BasicProgramSetup);
    program.with_dispatcher(CompletionDispatcher);
    program.with_dispatchers((AddDispatcherCommand, RemoveDispatcherCommand));

    program.exec();
}

gen_program!();
