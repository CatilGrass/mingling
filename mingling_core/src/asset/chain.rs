use std::fmt::Display;

use crate::ChainProcess;

#[doc(hidden)]
pub mod error;

/// Takes over a type (G: Previous) and converts it to another [AnyOutput](./struct.AnyOutput.html)
pub trait Chain<G>
where
    G: Display,
{
    /// The previous type in the chain
    type Previous;

    /// Process the previous value and return a future that resolves to a [`ChainProcess<G>`](./enum.ChainProcess.html)
    fn proc(p: Self::Previous) -> impl Future<Output = ChainProcess<G>> + Send;
}
