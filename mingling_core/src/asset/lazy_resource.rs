use crate::{ProgramCollect, ResourceMarker, this};

enum LazyInner<T> {
    /// Not yet initialized — holds the factory function.
    /// After init, the factory is **dropped**, no wasted memory.
    Uninit(Box<dyn FnMut() -> T + Send + Sync>),
    /// Initialized and ready to go.
    Init(T),
}

/// A lazily initialized resource that only creates its value on first access.
///
/// Unlike `Option<T>` + persistent `Box<dyn FnMut>`, the factory function is
/// **consumed and dropped** after initialization. No leftover allocation.
///
/// Initialization is triggered by `get_ref()`, `get_mut()`, or `get_clone()`.
pub struct LazyRes<T: Send + Sync + 'static> {
    inner: LazyInner<T>,
}

impl<T: Send + Sync + 'static> LazyRes<T> {
    /// Creates a new lazily initialized resource with a custom initializer.
    ///
    /// The factory `f` is called on first access, then dropped.
    #[must_use]
    pub fn new(f: impl FnMut() -> T + Send + Sync + 'static) -> Self {
        Self {
            inner: LazyInner::Uninit(Box::new(f)),
        }
    }

    fn force_init(&mut self) -> &mut LazyInner<T> {
        if matches!(&self.inner, LazyInner::Uninit(_)) {
            // Replace with a temporary poison value so we can move the real factory out.
            // SAFETY: if the factory panics, the poison value's `unreachable!()` will
            // catch any subsequent access.
            let poisoned = LazyInner::Uninit(Box::new(|| {
                unreachable!("LazyRes poisoned during initialization")
            }));
            let old = std::mem::replace(&mut self.inner, poisoned);
            match old {
                LazyInner::Uninit(mut f) => {
                    self.inner = LazyInner::Init((f)());
                }
                // Already initialized — put it back
                init @ LazyInner::Init(_) => {
                    self.inner = init;
                }
            }
        }
        &mut self.inner
    }

    /// Returns an immutable reference to the inner value,
    /// calling the initializer on first access.
    pub fn get_ref(&mut self) -> &T {
        self.force_init();
        match &self.inner {
            LazyInner::Init(t) => t,
            _ => unreachable!(),
        }
    }

    /// Returns a mutable reference to the inner value,
    /// calling the initializer on first access.
    pub fn get_mut(&mut self) -> &mut T {
        self.force_init();
        match &mut self.inner {
            LazyInner::Init(t) => t,
            _ => unreachable!(),
        }
    }

    /// Returns a clone of the inner value, calling the initializer if necessary.
    pub fn get_clone(&mut self) -> T
    where
        T: Clone,
    {
        self.get_ref().clone()
    }

    /// Consumes the lazy resource and returns the inner value, if initialized.
    ///
    /// Unlike `reset()`, this **drops** the lazy wrapper entirely.
    /// If you need to re-initialize, just construct a new `LazyRes::new(f)`.
    pub fn into_inner(self) -> Option<T> {
        match self.inner {
            LazyInner::Init(t) => Some(t),
            LazyInner::Uninit(_) => None,
        }
    }
}

impl<T: Send + Sync + 'static> Default for LazyRes<T>
where
    T: Default,
{
    /// Creates an uninitialized `LazyRes<T>` whose initializer returns `T::default()`.
    fn default() -> Self {
        Self::new(|| T::default())
    }
}

impl<T: Send + Sync + 'static> From<T> for LazyRes<T> {
    /// Creates a `LazyRes<T>` from an already-initialized value.
    fn from(value: T) -> Self {
        Self {
            inner: LazyInner::Init(value),
        }
    }
}

impl<T: Send + Sync + 'static> LazyRes<T> {
    /// Creates a lazily initialized resource using `T::default()` as the initializer.
    pub fn lazy_default() -> Self
    where
        T: Default,
    {
        Self::default()
    }

    /// Creates a lazily initialized resource with a custom initializer.
    ///
    /// Same as `LazyRes::new`.
    pub fn lazy_init(f: impl FnMut() -> T + Send + Sync + 'static) -> Self {
        Self::new(f)
    }
}

/// Provides convenience methods for types, allowing them to easily
/// create a corresponding `LazyRes<T>`.
pub trait LazyInit: Send + Sync + 'static {
    /// Creates a lazily initialized resource for this type using `Default` as the initializer.
    fn lazy_default() -> LazyRes<Self>
    where
        Self: Default,
    {
        LazyRes::default()
    }

    /// Creates a lazily initialized resource for this type with a custom initializer.
    fn lazy_init(f: impl FnMut() -> Self + Send + Sync + 'static) -> LazyRes<Self>
    where
        Self: Sized,
    {
        LazyRes::new(f)
    }
}

impl<T: Send + Sync + 'static> LazyInit for T {}

impl<T: Send + Sync + 'static> ResourceMarker for LazyRes<T>
where
    T: Default + Clone,
{
    /// Clones the lazy resource. The cloned resource retains any initialized value,
    /// but the initializer is reset to `T::default()`.
    fn res_clone(&self) -> Self {
        match &self.inner {
            LazyInner::Init(t) => Self {
                inner: LazyInner::Init(t.clone()),
            },
            LazyInner::Uninit(_) => Self::default(),
        }
    }

    /// Returns a default lazy resource (uninitialized, using `T::default()` as the initializer).
    fn res_default() -> Self
    where
        T: Default,
    {
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
