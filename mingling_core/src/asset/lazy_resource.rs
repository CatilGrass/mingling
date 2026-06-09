use crate::{ProgramCollect, ResourceMarker, this};

enum LazyInner<T> {
    /// Not yet initialized — holds the factory function.
    /// After init, the factory is **dropped**, no wasted memory.
    Uninit(
        Box<dyn FnMut() -> T + Send + Sync>,
        Option<Box<dyn FnOnce(T) + Send + Sync>>,
    ),
    /// Initialized and ready to go.
    Init(T, Option<Box<dyn FnOnce(T) + Send + Sync>>),
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
            inner: LazyInner::Uninit(Box::new(f), None),
        }
    }

    /// Creates a new lazily initialized resource with a custom initializer and
    /// an optional on-drop callback that receives ownership of the inner value when
    /// the `LazyRes` is dropped.
    #[must_use]
    pub fn new_with_drop(
        f: impl FnMut() -> T + Send + Sync + 'static,
        on_drop: impl FnOnce(T) + Send + Sync + 'static,
    ) -> Self {
        Self {
            inner: LazyInner::Uninit(Box::new(f), Some(Box::new(on_drop))),
        }
    }

    /// Chains an on-drop callback onto the lazy resource, returning ownership.
    ///
    /// This method consumes `self` and returns it with the callback attached,
    /// allowing a builder‑style pattern.
    ///
    /// The callback receives ownership of the inner value when the `LazyRes` is
    /// dropped. If the resource has not yet been initialized, the callback is stored
    /// and will be invoked once initialization happens *and* the `LazyRes` is later
    /// dropped.
    pub fn with_on_drop(mut self, on_drop: impl FnOnce(T) + Send + Sync + 'static) -> Self {
        match &mut self.inner {
            LazyInner::Uninit(_, existing_opt) => {
                *existing_opt = Some(Box::new(on_drop));
            }
            LazyInner::Init(_, existing_opt) => {
                *existing_opt = Some(Box::new(on_drop));
            }
        }
        self
    }

    /// Sets or replaces the on-drop callback for the initialized value.
    /// The callback is called with ownership of the inner value when the `LazyRes` is dropped.
    /// If the resource has not been initialized yet, the callback is stored and applied
    /// upon initialization.
    pub fn set_on_drop(&mut self, on_drop: impl FnOnce(T) + Send + Sync + 'static) {
        match &mut self.inner {
            LazyInner::Uninit(_, existing_opt) => {
                *existing_opt = Some(Box::new(on_drop));
            }
            LazyInner::Init(_, existing_opt) => {
                *existing_opt = Some(Box::new(on_drop));
            }
        }
    }

    /// Returns `true` if the resource has been initialized.
    pub fn is_initialized(&self) -> bool {
        matches!(&self.inner, LazyInner::Init(_, _))
    }

    fn force_init(&mut self) -> &mut LazyInner<T> {
        if matches!(&self.inner, LazyInner::Uninit(_, _)) {
            // Replace with a temporary poison value so the real factory can be moved out.
            // If the factory panics, the poison value's `unreachable!()` will
            // catch any subsequent access.
            let poisoned = LazyInner::Uninit(
                Box::new(|| unreachable!("LazyRes poisoned during initialization")),
                None,
            );
            let old = std::mem::replace(&mut self.inner, poisoned);
            match old {
                LazyInner::Uninit(mut f, on_drop) => {
                    self.inner = LazyInner::Init((f)(), on_drop);
                }
                // Already initialized — put it back
                init @ LazyInner::Init(_, _) => {
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
            LazyInner::Init(t, _) => t,
            _ => unreachable!(),
        }
    }

    /// Returns a mutable reference to the inner value,
    /// calling the initializer on first access.
    pub fn get_mut(&mut self) -> &mut T {
        self.force_init();
        match &mut self.inner {
            LazyInner::Init(t, _) => t,
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
    pub fn into_inner(mut self) -> Option<T> {
        // Take a temporary replacement to avoid moving out of a Drop type.
        let inner = std::mem::replace(
            &mut self.inner,
            LazyInner::Uninit(Box::new(|| unreachable!()), None),
        );
        match inner {
            LazyInner::Init(t, _) => Some(t),
            LazyInner::Uninit(_, _) => None,
        }
    }

    /// Consumes the lazy resource and returns the inner value.
    ///
    /// If the resource has not been initialized, the initializer is called first.
    /// This is different from `into_inner()` which returns `None` if uninitialized.
    pub fn unwrap(mut self) -> T {
        match std::mem::replace(
            &mut self.inner,
            LazyInner::Uninit(Box::new(|| unreachable!()), None),
        ) {
            LazyInner::Init(t, _) => t,
            LazyInner::Uninit(_, _) => {
                panic!("called `LazyRes::unwrap()` on an uninitialized value")
            }
        }
    }

    /// Consumes the lazy resource, calling the initializer first if not yet initialized.
    pub fn unwrap_or(mut self, default: T) -> T {
        if matches!(&self.inner, LazyInner::Uninit(_, _)) {
            default
        } else {
            match std::mem::replace(
                &mut self.inner,
                LazyInner::Uninit(Box::new(|| unreachable!()), None),
            ) {
                LazyInner::Init(t, _) => t,
                _ => unreachable!(),
            }
        }
    }

    /// Consumes the lazy resource, calling the initializer first if not yet initialized.
    ///
    /// If the resource has not been initialized, `T::default()` is used as the fallback.
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        self.unwrap_or(T::default())
    }
}

impl<T: Send + Sync + 'static> Drop for LazyRes<T> {
    fn drop(&mut self) {
        // Take ownership of the inner value and call the on-drop callback if present.
        let poisoned = LazyInner::Uninit(
            Box::new(|| unreachable!("LazyRes poisoned during drop")),
            None,
        );
        let old = std::mem::replace(&mut self.inner, poisoned);
        match old {
            LazyInner::Init(val, Some(callback)) => {
                callback(val);
            }
            LazyInner::Uninit(_, Some(_callback)) => {
                // Resource was never initialized but a drop callback was set.
                // Nothing to pass to the callback — just drop it.
            }
            _ => {}
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
            inner: LazyInner::Init(value, None),
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
            LazyInner::Init(t, _) => Self {
                inner: LazyInner::Init(t.clone(), None),
            },
            LazyInner::Uninit(_, _) => Self::default(),
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
