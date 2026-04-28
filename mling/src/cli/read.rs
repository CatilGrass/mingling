use colored::Colorize;
use std::path::PathBuf;

use mingling::{
    Groupped,
    macros::{chain, dispatcher, pack, r_println, renderer},
};
use serde::Serialize;

use crate::{
    ThisProgram,
    project_solver::{BinaryItem, solve_current_dir},
};

dispatcher!("show-target-dir", ReadTargetDirCommand => ReadTargetDirEntry);
dispatcher!("show-workspace-root", ReadWorkspaceRootCommand => ReadWorkspaceRootEntry);
dispatcher!("show-binaries", ReadBinariesCommand => ReadBinariesEntry);

pack!(ResultDir = PathBuf);
pack!(ResultTargetDirNotFound = ());

#[derive(Debug, Serialize, Default, Groupped)]
pub(crate) struct ResultBinaries {
    bin: Vec<BinaryItem>,
}

#[chain]
pub(crate) fn handle_target_dir_entry(_prev: ReadTargetDirEntry) -> NextProcess {
    match solve_current_dir() {
        Ok(solved) => {
            let dir = solved.target_dir;
            ResultDir::new(dir).to_render()
        }
        Err(_) => ResultTargetDirNotFound::new(()).to_render(),
    }
}

#[chain]
pub(crate) fn handle_workspace_root_entry(_prev: ReadWorkspaceRootEntry) -> NextProcess {
    match solve_current_dir() {
        Ok(solved) => {
            let dir = solved.workspace_root;
            ResultDir::new(dir).to_render()
        }
        Err(_) => ResultTargetDirNotFound::new(()).to_render(),
    }
}

#[chain]
pub(crate) fn handle_binaries_entry(_prev: ReadBinariesEntry) -> NextProcess {
    match solve_current_dir() {
        Ok(solved) => {
            let binaries = solved.binaries;
            ResultBinaries { bin: binaries }.to_render()
        }
        Err(_) => ResultTargetDirNotFound::new(()).to_render(),
    }
}

#[renderer]
pub(crate) fn render_dir(prev: ResultDir) {
    r_println!("{}", prev.inner.display())
}

#[renderer]
pub(crate) fn render_binaries(prev: ResultBinaries) {
    let mut i = 1;
    for item in prev.bin.iter() {
        r_println!(
            "{}. {} ({})",
            i.to_string(),
            item.name.bold(),
            item.path.to_string_lossy().underline().bright_cyan()
        );
        i += 1;
    }
}
