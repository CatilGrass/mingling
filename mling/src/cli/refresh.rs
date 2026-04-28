use mingling::{
    ShellContext, Suggest,
    macros::{chain, completion, dispatcher, pack, suggest},
    parser::Picker,
};

use crate::{ThisProgram, project_installer::install_all};

dispatcher!("refresh", RefreshCommand => RefreshEntry);

pack!(ResultRefreshCompleted = ());

#[completion(RefreshEntry)]
pub(crate) fn comp_refresh(ctx: &ShellContext) -> Suggest {
    if ctx.typing_argument() {
        return suggest! {
            "--clean": "Clean build artifacts before installation",
            "-c": "Clean build artifacts before installation",
        };
    }
    return suggest!();
}

#[chain]
pub(crate) fn handle_refresh_entry(prev: RefreshEntry) -> NextProcess {
    let is_clean_before_build = Picker::new(prev.inner)
        .pick::<bool>(["--clean", "-c"])
        .unpack();
    let _ = install_all(is_clean_before_build);

    ResultRefreshCompleted::new(())
}
