use mingling::{
    macros::{r_println, renderer},
    setup::{BasicProgramSetup, GeneralRendererSetup},
};

use crate::{
    __completion_gen::CompletionDispatcher, DispatcherNotFound, ThisProgram, display::markdown,
};

pub mod list;
pub use list::*;

pub mod namespace_mgr;
pub use namespace_mgr::*;

pub mod read;
pub use read::*;

pub mod install;
pub use install::*;

pub fn cli_entry() {
    let mut program = ThisProgram::new();

    // Plugins
    program.with_setup(BasicProgramSetup);
    program.with_setup(GeneralRendererSetup);
    program.with_dispatcher(CompletionDispatcher);

    if program.pick_global_flag(["-v", "--version"]) {
        println!("{}", include_str!("../res/version.txt").trim_end());
        return;
    }

    // Help
    if program.user_context.help {
        println!(
            "{}",
            markdown(include_str!("../res/help-mling.txt").trim_end())
        );
        return;
    }

    // Context query commands
    program.with_dispatcher(ListInstalledCommand);
    program.with_dispatchers((
        ReadTargetDirCommand,
        ReadWorkspaceRootCommand,
        ReadBinariesCommand,
    ));

    // Namespace manage commands
    program.with_dispatchers((
        TrustNamespaceCommand,
        UntrustNamespaceCommand,
        SetTrustNamespaceCommand,
        RemoveNamespaceCommand,
    ));

    // Install binaries command
    program.with_dispatcher(InstallCommand);

    // Colored Setup
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    program.exec();
}

#[renderer]
pub(crate) fn fallback_disp(prev: DispatcherNotFound) {
    r_println!("Error: command \"{}\" not found!", prev.join(" "));
    r_println!("Use \"mling --help\" for more information.");
}
