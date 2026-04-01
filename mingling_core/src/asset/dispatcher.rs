use std::fmt::Display;

use crate::{ChainProcess, Program, asset::node::Node};

pub trait Dispatcher<G>
where
    G: Display,
{
    fn node(&self) -> Node;
    fn begin(&self, args: Vec<String>) -> ChainProcess<G>;
    fn clone_dispatcher(&self) -> Box<dyn Dispatcher<G>>;
}

impl<G> Clone for Box<dyn Dispatcher<G>>
where
    G: Display,
{
    fn clone(&self) -> Self {
        self.clone_dispatcher()
    }
}

impl<C: crate::program::ProgramCollect, G: Display> Program<C, G> {
    /// Adds a dispatcher to the program.
    pub fn with_dispatcher<Disp>(&mut self, dispatcher: Disp)
    where
        Disp: Dispatcher<G> + 'static,
    {
        self.dispatcher.push(Box::new(dispatcher));
    }

    /// Add some dispatchers to the program.
    pub fn with_dispatchers<D>(&mut self, dispatchers: D)
    where
        D: Into<Dispatchers<G>>,
    {
        let dispatchers = dispatchers.into();
        self.dispatcher.extend(dispatchers.dispatcher);
    }
}

pub struct Dispatchers<G> {
    dispatcher: Vec<Box<dyn Dispatcher<G> + 'static>>,
}

impl<G> From<Vec<Box<dyn Dispatcher<G>>>> for Dispatchers<G> {
    fn from(dispatcher: Vec<Box<dyn Dispatcher<G>>>) -> Self {
        Self { dispatcher }
    }
}

impl<G> From<Box<dyn Dispatcher<G>>> for Dispatchers<G> {
    fn from(dispatcher: Box<dyn Dispatcher<G>>) -> Self {
        Self {
            dispatcher: vec![dispatcher],
        }
    }
}

impl<D, G> From<(D,)> for Dispatchers<G>
where
    D: Dispatcher<G> + 'static,
    G: Display,
{
    fn from(dispatcher: (D,)) -> Self {
        Self {
            dispatcher: vec![Box::new(dispatcher.0)],
        }
    }
}

impl<D1, D2, G> From<(D1, D2)> for Dispatchers<G>
where
    D1: Dispatcher<G> + 'static,
    D2: Dispatcher<G> + 'static,
    G: Display,
{
    fn from(dispatchers: (D1, D2)) -> Self {
        Self {
            dispatcher: vec![Box::new(dispatchers.0), Box::new(dispatchers.1)],
        }
    }
}

impl<D1, D2, D3, G> From<(D1, D2, D3)> for Dispatchers<G>
where
    D1: Dispatcher<G> + 'static,
    D2: Dispatcher<G> + 'static,
    D3: Dispatcher<G> + 'static,
    G: Display,
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

impl<D1, D2, D3, D4, G> From<(D1, D2, D3, D4)> for Dispatchers<G>
where
    D1: Dispatcher<G> + 'static,
    D2: Dispatcher<G> + 'static,
    D3: Dispatcher<G> + 'static,
    D4: Dispatcher<G> + 'static,
    G: Display,
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

impl<D1, D2, D3, D4, D5, G> From<(D1, D2, D3, D4, D5)> for Dispatchers<G>
where
    D1: Dispatcher<G> + 'static,
    D2: Dispatcher<G> + 'static,
    D3: Dispatcher<G> + 'static,
    D4: Dispatcher<G> + 'static,
    D5: Dispatcher<G> + 'static,
    G: Display,
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

impl<D1, D2, D3, D4, D5, D6, G> From<(D1, D2, D3, D4, D5, D6)> for Dispatchers<G>
where
    D1: Dispatcher<G> + 'static,
    D2: Dispatcher<G> + 'static,
    D3: Dispatcher<G> + 'static,
    D4: Dispatcher<G> + 'static,
    D5: Dispatcher<G> + 'static,
    D6: Dispatcher<G> + 'static,
    G: Display,
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

impl<D1, D2, D3, D4, D5, D6, D7, G> From<(D1, D2, D3, D4, D5, D6, D7)> for Dispatchers<G>
where
    D1: Dispatcher<G> + 'static,
    D2: Dispatcher<G> + 'static,
    D3: Dispatcher<G> + 'static,
    D4: Dispatcher<G> + 'static,
    D5: Dispatcher<G> + 'static,
    D6: Dispatcher<G> + 'static,
    D7: Dispatcher<G> + 'static,
    G: Display,
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

impl<G> std::ops::Deref for Dispatchers<G> {
    type Target = Vec<Box<dyn Dispatcher<G> + 'static>>;

    fn deref(&self) -> &Self::Target {
        &self.dispatcher
    }
}

impl<G> From<Dispatchers<G>> for Vec<Box<dyn Dispatcher<G> + 'static>> {
    fn from(val: Dispatchers<G>) -> Self {
        val.dispatcher
    }
}
