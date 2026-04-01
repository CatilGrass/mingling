use std::fmt::Display;

#[cfg(feature = "general_renderer")]
use serde::Serialize;

use crate::Groupped;
use crate::error::ChainProcessError;

pub mod group;

#[derive(Debug)]
pub struct AnyOutput<G>
where
    G: Display,
{
    inner: Box<dyn std::any::Any + Send + 'static>,
    pub type_id: std::any::TypeId,
    pub member_id: G,
}

impl<G> AnyOutput<G>
where
    G: Display,
{
    #[cfg(feature = "general_renderer")]
    pub fn new<T>(value: T) -> Self
    where
        T: Send + Groupped<G> + Serialize + 'static,
    {
        Self {
            inner: Box::new(value),
            type_id: std::any::TypeId::of::<T>(),
            member_id: T::member_id(),
        }
    }

    #[cfg(not(feature = "general_renderer"))]
    pub fn new<T>(value: T) -> Self
    where
        T: Send + Groupped<G> + 'static,
    {
        Self {
            inner: Box::new(value),
            type_id: std::any::TypeId::of::<T>(),
            member_id: T::member_id(),
        }
    }

    pub fn downcast<T: 'static>(self) -> Result<T, Self> {
        if self.type_id == std::any::TypeId::of::<T>() {
            Ok(*self.inner.downcast::<T>().unwrap())
        } else {
            Err(self)
        }
    }

    pub fn is<T: 'static>(&self) -> bool {
        self.type_id == std::any::TypeId::of::<T>()
    }

    /// Route the output to the next Chain
    pub fn route_chain(self) -> ChainProcess<G> {
        ChainProcess::Ok((self, Next::Chain))
    }

    /// Route the output to the Renderer, ending execution
    pub fn route_renderer(self) -> ChainProcess<G> {
        ChainProcess::Ok((self, Next::Renderer))
    }
}

impl<G> std::ops::Deref for AnyOutput<G>
where
    G: Display,
{
    type Target = dyn std::any::Any + Send + 'static;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl<G> std::ops::DerefMut for AnyOutput<G>
where
    G: Display,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.inner
    }
}

pub enum ChainProcess<G>
where
    G: Display,
{
    Ok((AnyOutput<G>, Next)),
    Err(ChainProcessError),
}

pub enum Next {
    Chain,
    Renderer,
}

impl<G> ChainProcess<G>
where
    G: Display,
{
    pub fn is_next(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        matches!(self, Self::Err(_))
    }

    pub fn next(&self) -> Option<&Next> {
        match self {
            Self::Ok((_, next)) => Some(next),
            Self::Err(_) => None,
        }
    }

    pub fn err(&self) -> Option<&ChainProcessError> {
        match self {
            Self::Ok(_) => None,
            Self::Err(err) => Some(err),
        }
    }

    pub fn unwrap(self) -> (AnyOutput<G>, Next) {
        match self {
            Self::Ok(tuple) => tuple,
            Self::Err(_) => panic!("called `ChainProcess2::unwrap()` on an `Error` value"),
        }
    }

    pub fn unwrap_or(self, default: (AnyOutput<G>, Next)) -> (AnyOutput<G>, Next) {
        match self {
            Self::Ok(tuple) => tuple,
            Self::Err(_) => default,
        }
    }

    pub fn unwrap_or_else<F>(self, f: F) -> (AnyOutput<G>, Next)
    where
        F: FnOnce(ChainProcessError) -> (AnyOutput<G>, Next),
    {
        match self {
            Self::Ok(tuple) => tuple,
            Self::Err(err) => f(err),
        }
    }
}
