use std::fmt::Display;

use crate::ChainProcess;

pub mod error;

pub trait Chain<G>
where
    G: Display,
{
    type Previous;
    fn proc(p: Self::Previous) -> impl Future<Output = ChainProcess<G>> + Send;
}
