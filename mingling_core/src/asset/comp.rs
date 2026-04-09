mod flags;
mod shell_ctx;
mod suggest;

#[doc(hidden)]
pub use flags::*;
#[doc(hidden)]
pub use shell_ctx::*;
#[doc(hidden)]
pub use suggest::*;

/// Trait for implementing completion logic.
///
/// This trait defines the interface for generating command-line completions.
/// Types implementing this trait can provide custom completion suggestions
/// based on the current shell context.
pub trait Completion {
    type Previous;
    fn comp(ctx: ShellContext) -> Suggest;
}
