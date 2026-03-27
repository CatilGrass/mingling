use crate::asset::dispatcher::Dispatcher;
use std::env;

pub mod setup;

mod config;
pub use config::*;

mod flag;
pub use flag::*;

#[derive(Default)]
pub struct Program {
    pub(crate) args: Vec<String>,
    pub(crate) dispatcher: Vec<Box<dyn Dispatcher>>,

    pub stdout_setting: ProgramStdoutSetting,
    pub user_context: ProgramUserContext,
}

impl Program {
    /// Creates a new Program instance, initializing args from environment.
    pub fn new() -> Self {
        Program {
            args: env::args().collect(),
            dispatcher: Vec::new(),
            ..Default::default()
        }
    }

    /// Run the command line program
    pub async fn exec(self) {
        todo!()
    }
}
