use mingling::{macros::gen_program, setup::BasicProgramSetup};

mod add_dispatcher;
pub use crate::add_dispatcher::*;

#[tokio::main]
async fn main() {
    let mut program = MinglingCLI::new();
    program.with_setup(BasicProgramSetup);
    program.with_dispatcher(AddDispatcherCommand);

    let render_result = program.exec_without_render().await.unwrap();
    println!("{}", render_result);
}

gen_program!(MinglingCLI);
