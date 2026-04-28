use mingling::{
    macros::renderer,
    setup::{BasicProgramSetup, GeneralRendererSetup},
};

use crate::{__completion_gen::CompletionDispatcher, DispatcherNotFound, ThisProgram};

pub mod list;
pub use list::*;

pub mod namespace_mgr;
pub use namespace_mgr::*;

pub mod read;
pub use read::*;

pub mod refresh;
pub use refresh::*;

pub fn cli_entry() {
    let mut program = ThisProgram::new();

    program.with_setup(BasicProgramSetup);
    program.with_setup(GeneralRendererSetup);
    program.with_dispatcher(CompletionDispatcher);

    program.with_dispatcher(ListInstalledCommand);
    program.with_dispatchers((
        TrustNamespaceCommand,
        UntrustNamespaceCommand,
        SetTrustNamespaceCommand,
        RemoveNamespaceCommand,
    ));
    program.with_dispatcher(RefreshCommand);
    program.with_dispatchers((
        ReadTargetDirCommand,
        ReadWorkspaceRootCommand,
        ReadBinariesCommand,
    ));

    program.exec();
}

#[renderer]
pub(crate) fn render_help(_prev: DispatcherNotFound) {}
