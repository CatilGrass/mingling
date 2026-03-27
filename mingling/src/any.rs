#[cfg(feature = "serde_renderer")]
use serde::Serialize;

use crate::error::ChainProcessError;

pub type ChainProcess = Result<AnyOutput, ChainProcessError>;

#[derive(Debug)]
pub struct AnyOutput {
    inner: Box<dyn std::any::Any + Send + 'static>,
    type_id: std::any::TypeId,
}

impl AnyOutput {
    #[cfg(feature = "serde_renderer")]
    pub fn new<T>(value: T) -> Self
    where
        T: Send + Serialize + 'static,
    {
        Self {
            inner: Box::new(value),
            type_id: std::any::TypeId::of::<T>(),
        }
    }

    #[cfg(not(feature = "serde_renderer"))]
    pub fn new<T>(value: T) -> Self
    where
        T: Send + 'static,
    {
        Self {
            inner: Box::new(value),
            type_id: std::any::TypeId::of::<T>(),
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
    pub fn route_chain(self) -> ChainProcess {
        Ok(self)
    }

    /// Route the output to the Renderer, ending execution
    pub fn route_renderer(self) -> ChainProcess {
        Err(ChainProcessError::Broken(self))
    }
}

impl std::ops::Deref for AnyOutput {
    type Target = dyn std::any::Any + Send + 'static;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl std::ops::DerefMut for AnyOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.inner
    }
}
