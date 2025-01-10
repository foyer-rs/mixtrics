/// Scoped functional programming extensions.
pub trait Scope {
    /// Scoped with ownership.
    fn with<F, R>(self, f: F) -> R
    where
        Self: Sized,
        F: FnOnce(Self) -> R,
    {
        f(self)
    }

    #[expect(unused)]
    /// Scoped with reference.
    fn with_ref<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Self) -> R,
    {
        f(self)
    }

    #[expect(unused)]
    /// Scoped with mutable reference.
    fn with_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        f(self)
    }
}
impl<T> Scope for T {}

/// Helper trait for boxing.
pub trait Boxer {
    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}
impl<T> Boxer for T {}
