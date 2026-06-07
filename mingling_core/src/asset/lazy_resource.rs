use crate::{ProgramCollect, ResourceMarker, this};

/// A lazily initialized resource that only creates its value on first access via a given initializer function.
///
/// `LazyRes<T>` wraps an `Option<T>` and an initializer function, supporting thread-safe lazy loading.
/// Initialization is triggered by calls to `get_ref()`, `get_mut()`, or `get_clone()`.
pub struct LazyRes<T: Default + Send + Sync + Clone> {
    /// The initializer function, called on first access to produce the inner value.
    init_fn: Box<dyn FnMut() -> T + Send + Sync>,
    /// Stores the initialized value; `None` means not yet initialized.
    inner: Option<T>,
}

impl<T: Default + Send + Sync + Clone> Default for LazyRes<T> {
    /// Creates an uninitialized `LazyRes<T>` whose initializer returns `T::default()`.
    fn default() -> Self {
        Self {
            inner: None,
            init_fn: Box::new(|| T::default()),
        }
    }
}

impl<T: Default + Send + Sync + Clone + 'static> LazyRes<T> {
    /// Creates a new lazily initialized resource with a custom initializer function.
    ///
    /// # Parameters
    /// - `f`: The initializer function called on first access, returning a value of type `T`.
    ///
    /// # Returns
    /// Returns an uninitialized `LazyRes<T>`.
    #[must_use]
    pub fn new(f: impl FnMut() -> T + Send + Sync + 'static) -> Self {
        Self {
            inner: None,
            init_fn: Box::new(f),
        }
    }

    /// Returns an immutable reference to the inner value, calling the initializer if necessary.
    ///
    /// # Returns
    /// An immutable reference to the inner value `T`.
    pub fn get_ref(&mut self) -> &T {
        if self.inner.is_none() {
            self.inner = Some((self.init_fn)());
        }
        self.inner.as_ref().unwrap()
    }

    /// Returns a mutable reference to the inner value, calling the initializer if necessary.
    ///
    /// # Returns
    /// A mutable reference to the inner value `T`.
    pub fn get_mut(&mut self) -> &mut T {
        if self.inner.is_none() {
            self.inner = Some((self.init_fn)());
        }
        self.inner.as_mut().unwrap()
    }

    /// Resets the lazy resource by taking and returning the inner value; on next access, re-initialization occurs.
    ///
    /// # Returns
    /// Returns the previously initialized value if any, otherwise `None`.
    pub fn reset(&mut self) -> Option<T> {
        self.inner.take()
    }

    /// Returns a clone of the inner value, calling the initializer if necessary.
    ///
    /// # Returns
    /// A clone of type `T`.
    pub fn get_clone(&mut self) -> T {
        self.get_ref().clone()
    }
}

impl<T: Default + Send + Sync + Clone + 'static> From<T> for LazyRes<T> {
    /// Creates a `LazyRes<T>` from an existing value (already initialized), with the initializer set to `T::default()`.
    fn from(value: T) -> Self {
        Self {
            inner: Some(value),
            init_fn: Box::new(|| T::default()),
        }
    }
}

impl<T: Default + Send + Sync + Clone + 'static> LazyRes<T> {
    /// Creates a lazily initialized resource using `T::default()` as the initializer.
    pub fn lazy_default() -> Self {
        Self::default()
    }

    /// Creates a lazily initialized resource with a custom initializer function.
    ///
    /// Same as `LazyRes::new`.
    pub fn lazy_init(f: impl FnMut() -> T + Send + Sync + 'static) -> Self {
        Self::new(f)
    }
}

/// Provides convenience methods for types implementing `Default + Send + Sync + Clone + 'static`,
/// allowing them to easily create a corresponding `LazyRes<T>`.
pub trait LazyInit: Default + Send + Sync + Clone + 'static {
    /// Creates a lazily initialized resource for this type using `Default` as the initializer.
    fn lazy_default() -> LazyRes<Self> {
        LazyRes::default()
    }

    /// Creates a lazily initialized resource for this type with a custom initializer function.
    fn lazy_init(f: impl FnMut() -> Self + Send + Sync + 'static) -> LazyRes<Self> {
        LazyRes::new(f)
    }
}

impl<T: Default + Send + Sync + Clone + 'static> LazyInit for T {}

impl<T: Default + Send + Sync + Clone + 'static> ResourceMarker for LazyRes<T> {
    /// Clones the lazy resource. The cloned resource retains any initialized value, but the initializer is reset to `T::default()`.
    fn res_clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            // The original initializer is not preserved on clone
            init_fn: Box::new(|| T::default()),
        }
    }

    /// Returns a default lazy resource (uninitialized, using `T::default()` as the initializer).
    fn res_default() -> Self {
        Self::default()
    }

    /// Modifies the current lazy resource via the `this` context provided by `C`.
    fn modify<C>(f: impl FnOnce(&mut Self))
    where
        C: ProgramCollect<Enum = C> + 'static,
    {
        this::<C>().modify_res(f);
    }
}
