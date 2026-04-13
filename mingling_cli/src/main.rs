use mingling::{macros::gen_program, setup::BasicProgramSetup};

mod dispatcher_mgr;
pub use crate::dispatcher_mgr::*;

#[tokio::main]
async fn main() {
    let mut program = ThisProgram::new();
    program.with_setup(BasicProgramSetup);
    program.with_dispatcher(CompletionDispatcher);
    program.with_dispatchers((AddDispatcherCommand, RemoveDispatcherCommand));

    program.exec().await;
}

gen_program!();
