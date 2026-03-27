use crate::{ChainProcess, Program, asset::node::Node};

pub use mingling_macros::Dispatcher;

pub trait Dispatcher {
    fn node(&self) -> Node;
}

pub trait DispatcherChain {
    fn begin(&self) -> ChainProcess;
}

impl Program {
    /// Adds a dispatcher to the program.
    pub fn with_dispatcher<D: Dispatcher + 'static>(&mut self, dispatcher: D) {
        let dispatcher = Box::new(dispatcher);
        self.dispatcher.push(dispatcher);
    }
}
