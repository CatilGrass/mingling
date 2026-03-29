use crate::{ChainProcess, Program, asset::node::Node};

pub trait Dispatcher {
    fn node(&self) -> Node;
    fn begin(&self, args: Vec<String>) -> ChainProcess;
    fn clone_dispatcher(&self) -> Box<dyn Dispatcher>;
}

impl Clone for Box<dyn Dispatcher> {
    fn clone(&self) -> Self {
        self.clone_dispatcher()
    }
}

impl<C: crate::program::ProgramCollect> Program<C> {
    /// Adds a dispatcher to the program.
    pub fn with_dispatcher<D>(&mut self, dispatcher: D)
    where
        D: Into<Dispatchers>,
    {
        let dispatchers = dispatcher.into().dispatcher;
        self.dispatcher.extend(dispatchers);
    }
}

pub struct Dispatchers {
    dispatcher: Vec<Box<dyn Dispatcher + 'static>>,
}

impl<D> From<D> for Dispatchers
where
    D: Dispatcher + 'static,
{
    fn from(dispatcher: D) -> Self {
        Self {
            dispatcher: vec![Box::new(dispatcher)],
        }
    }
}

impl From<Vec<Box<dyn Dispatcher>>> for Dispatchers {
    fn from(dispatcher: Vec<Box<dyn Dispatcher>>) -> Self {
        Self { dispatcher }
    }
}

impl From<Box<dyn Dispatcher>> for Dispatchers {
    fn from(dispatcher: Box<dyn Dispatcher>) -> Self {
        Self {
            dispatcher: vec![dispatcher],
        }
    }
}

impl<D> From<(D,)> for Dispatchers
where
    D: Dispatcher + 'static,
{
    fn from(dispatcher: (D,)) -> Self {
        Self {
            dispatcher: vec![Box::new(dispatcher.0)],
        }
    }
}

impl<D1, D2> From<(D1, D2)> for Dispatchers
where
    D1: Dispatcher + 'static,
    D2: Dispatcher + 'static,
{
    fn from(dispatchers: (D1, D2)) -> Self {
        Self {
            dispatcher: vec![Box::new(dispatchers.0), Box::new(dispatchers.1)],
        }
    }
}

impl<D1, D2, D3> From<(D1, D2, D3)> for Dispatchers
where
    D1: Dispatcher + 'static,
    D2: Dispatcher + 'static,
    D3: Dispatcher + 'static,
{
    fn from(dispatchers: (D1, D2, D3)) -> Self {
        Self {
            dispatcher: vec![
                Box::new(dispatchers.0),
                Box::new(dispatchers.1),
                Box::new(dispatchers.2),
            ],
        }
    }
}

impl<D1, D2, D3, D4> From<(D1, D2, D3, D4)> for Dispatchers
where
    D1: Dispatcher + 'static,
    D2: Dispatcher + 'static,
    D3: Dispatcher + 'static,
    D4: Dispatcher + 'static,
{
    fn from(dispatchers: (D1, D2, D3, D4)) -> Self {
        Self {
            dispatcher: vec![
                Box::new(dispatchers.0),
                Box::new(dispatchers.1),
                Box::new(dispatchers.2),
                Box::new(dispatchers.3),
            ],
        }
    }
}

impl<D1, D2, D3, D4, D5> From<(D1, D2, D3, D4, D5)> for Dispatchers
where
    D1: Dispatcher + 'static,
    D2: Dispatcher + 'static,
    D3: Dispatcher + 'static,
    D4: Dispatcher + 'static,
    D5: Dispatcher + 'static,
{
    fn from(dispatchers: (D1, D2, D3, D4, D5)) -> Self {
        Self {
            dispatcher: vec![
                Box::new(dispatchers.0),
                Box::new(dispatchers.1),
                Box::new(dispatchers.2),
                Box::new(dispatchers.3),
                Box::new(dispatchers.4),
            ],
        }
    }
}

impl<D1, D2, D3, D4, D5, D6> From<(D1, D2, D3, D4, D5, D6)> for Dispatchers
where
    D1: Dispatcher + 'static,
    D2: Dispatcher + 'static,
    D3: Dispatcher + 'static,
    D4: Dispatcher + 'static,
    D5: Dispatcher + 'static,
    D6: Dispatcher + 'static,
{
    fn from(dispatchers: (D1, D2, D3, D4, D5, D6)) -> Self {
        Self {
            dispatcher: vec![
                Box::new(dispatchers.0),
                Box::new(dispatchers.1),
                Box::new(dispatchers.2),
                Box::new(dispatchers.3),
                Box::new(dispatchers.4),
                Box::new(dispatchers.5),
            ],
        }
    }
}

impl<D1, D2, D3, D4, D5, D6, D7> From<(D1, D2, D3, D4, D5, D6, D7)> for Dispatchers
where
    D1: Dispatcher + 'static,
    D2: Dispatcher + 'static,
    D3: Dispatcher + 'static,
    D4: Dispatcher + 'static,
    D5: Dispatcher + 'static,
    D6: Dispatcher + 'static,
    D7: Dispatcher + 'static,
{
    fn from(dispatchers: (D1, D2, D3, D4, D5, D6, D7)) -> Self {
        Self {
            dispatcher: vec![
                Box::new(dispatchers.0),
                Box::new(dispatchers.1),
                Box::new(dispatchers.2),
                Box::new(dispatchers.3),
                Box::new(dispatchers.4),
                Box::new(dispatchers.5),
                Box::new(dispatchers.6),
            ],
        }
    }
}

impl std::ops::Deref for Dispatchers {
    type Target = Vec<Box<dyn Dispatcher + 'static>>;

    fn deref(&self) -> &Self::Target {
        &self.dispatcher
    }
}

impl From<Dispatchers> for Vec<Box<dyn Dispatcher + 'static>> {
    fn from(val: Dispatchers) -> Self {
        val.dispatcher
    }
}
