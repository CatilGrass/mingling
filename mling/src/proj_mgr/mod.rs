use crate::ThisProgram;
use mingling::{
    Program,
    macros::{dispatcher, program_setup},
};

pub mod metadata;

dispatcher!("show.binaries");
dispatcher!("show.workspace");
dispatcher!("show.target-dir",
    CMDShowTargetDirectories => EntryShowTargetDirectories
);

#[program_setup]
pub fn project_manager_setup(p: &mut Program<ThisProgram>) {
    p.with_dispatcher(CMDShowBinaries);
    p.with_dispatcher(CMDShowWorkspace);
    p.with_dispatcher(CMDShowTargetDirectories);
}
