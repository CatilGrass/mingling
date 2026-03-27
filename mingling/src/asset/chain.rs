use crate::ChainProcess;

pub mod error;

pub trait Chain {
    type Previous;
    fn proc(p: Self::Previous) -> impl Future<Output = ChainProcess> + Send;
}
