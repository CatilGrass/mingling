use std::sync::OnceLock;

use crate::{Program, ProgramCollect};

/// Global static reference to the current program instance
pub(crate) static THIS_PROGRAM: OnceLock<Option<Box<dyn std::any::Any + Send + Sync>>> =
    OnceLock::new();

/// Returns a reference to the current program instance, panics if not set.
///
/// # Panics
///
/// Panics if the program has not been initialized yet.
#[must_use]
pub fn this<C>() -> &'static Program<C>
where
    C: ProgramCollect<Enum = C> + 'static,
{
    try_get_this_program().expect("Program not initialized")
}

/// Returns a reference to the current program instance, if set.
fn try_get_this_program<C>() -> Option<&'static Program<C>>
where
    C: ProgramCollect<Enum = C> + 'static,
{
    THIS_PROGRAM.get()?.as_ref()?.downcast_ref::<Program<C>>()
}
