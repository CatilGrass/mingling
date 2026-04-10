mod flags;
mod shell_ctx;
mod suggest;

use std::fmt::Display;

#[doc(hidden)]
pub use flags::*;
#[doc(hidden)]
pub use shell_ctx::*;
#[doc(hidden)]
pub use suggest::*;

use crate::{ProgramCollect, this};

/// Trait for implementing completion logic.
///
/// This trait defines the interface for generating command-line completions.
/// Types implementing this trait can provide custom completion suggestions
/// based on the current shell context.
pub trait Completion {
    type Previous;
    fn comp(ctx: &ShellContext) -> Suggest;
}

/// Trait for extracting user input arguments for completion.
///
/// When the `feat comp` feature is enabled, the `dispatcher!` macro will
/// automatically implement this trait for `Entry` types to extract the
/// arguments from user input for completion suggestions.
pub trait CompletionEntry {
    fn get_input(self) -> Vec<String>;
}

pub struct CompletionHelper;
impl CompletionHelper {
    pub fn exec_completion<P>(ctx: &ShellContext) -> Suggest
    where
        P: ProgramCollect + Display + 'static,
    {
        let program = this::<P>();
        Suggest::FileCompletion
    }

    pub fn render_suggest<P>(ctx: ShellContext, suggest: Suggest)
    where
        P: ProgramCollect + Display + 'static,
    {
    }
}
