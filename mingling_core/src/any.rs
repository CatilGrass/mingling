use std::fmt::Display;

#[cfg(feature = "general_renderer")]
use serde::Serialize;

use crate::Groupped;
use crate::error::ChainProcessError;

#[doc(hidden)]
pub mod group;

/// Any type output
///
/// Accepts any type that implements `Send + Groupped<G>`
/// After being passed into AnyOutput, it will be converted to `Box<dyn Any + Send + 'static>`
///
/// Note:
/// - If an enum value that does not belong to this type is incorrectly specified, it will be **unsafely** unwrapped by the scheduler
/// - Under the `general_renderer` feature, the passed value must ensure it implements `serde::Serialize`
/// - It is recommended to use the `pack!` macro from [mingling_macros](https://crates.io/crates/mingling_macros) to create types that can be converted to `AnyOutput`, which guarantees runtime safety
#[derive(Debug)]
pub struct AnyOutput<G>
where
    G: Display,
{
    pub(crate) inner: Box<dyn std::any::Any + Send + 'static>,
    pub type_id: std::any::TypeId,
    pub member_id: G,
}

impl<G> AnyOutput<G>
where
    G: Display,
{
    /// Create an AnyOutput from a `Send + Groupped<G> + Serialize` type
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

    /// Create an AnyOutput from a `Send + Groupped<G>` type
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

    /// Downcast the AnyOutput to a concrete type T
    pub fn downcast<T: 'static>(self) -> Result<T, Self> {
        if self.type_id == std::any::TypeId::of::<T>() {
            Ok(*self.inner.downcast::<T>().unwrap())
        } else {
            Err(self)
        }
    }

    /// Check if the inner value is of type T
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

    #[cfg(feature = "general_renderer")]
    /// Restore AnyOutput back to the original Serialize type
    pub fn restore<T: Serialize + 'static>(self) -> Option<T> {
        if self.type_id == std::any::TypeId::of::<T>() {
            match self.inner.downcast::<T>() {
                Ok(boxed) => Some(*boxed),
                Err(_) => None,
            }
        } else {
            None
        }
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

/// Chain exec result type
///
/// Stores `Ok` and `Err` types of execution results, used to notify the scheduler what to execute next
/// - Returns `Ok((`[`AnyOutput`](./struct.AnyOutput.html)`, `[`Next::Chain`](./enum.Next.html)`))` to continue execution with this type next
/// - Returns `Ok((`[`AnyOutput`](./struct.AnyOutput.html)`, `[`Next::Renderer`](./enum.Next.html)`))` to render this type next and output to the terminal
/// - Returns `Err(`[`ChainProcessError`](./error/enum.ChainProcessError.html)`]` to terminate the program directly
pub enum ChainProcess<G>
where
    G: Display,
{
    Ok((AnyOutput<G>, Next)),
    Err(ChainProcessError),
}

/// Indicates the next step after processing
///
/// - `Chain`: Continue execution to the next chain
/// - `Renderer`: Send output to renderer and end execution
pub enum Next {
    Chain,
    Renderer,
}

impl<G> ChainProcess<G>
where
    G: Display,
{
    /// Returns true if the result is Ok (has a next step)
    pub fn is_next(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    /// Returns true if the result is an error
    pub fn is_err(&self) -> bool {
        matches!(self, Self::Err(_))
    }

    /// Returns the next step if the result is Ok
    pub fn next(&self) -> Option<&Next> {
        match self {
            Self::Ok((_, next)) => Some(next),
            Self::Err(_) => None,
        }
    }

    /// Returns the error if the result is Err
    pub fn err(&self) -> Option<&ChainProcessError> {
        match self {
            Self::Ok(_) => None,
            Self::Err(err) => Some(err),
        }
    }

    /// Unwraps the result, panics if it's an error
    pub fn unwrap(self) -> (AnyOutput<G>, Next) {
        match self {
            Self::Ok(tuple) => tuple,
            Self::Err(_) => panic!("called `ChainProcess2::unwrap()` on an `Error` value"),
        }
    }

    /// Returns the Ok value or a provided default
    pub fn unwrap_or(self, default: (AnyOutput<G>, Next)) -> (AnyOutput<G>, Next) {
        match self {
            Self::Ok(tuple) => tuple,
            Self::Err(_) => default,
        }
    }

    /// Returns the Ok value or computes it from the error
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
