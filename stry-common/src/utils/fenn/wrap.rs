/// A trait that allows for creating 'builder' like chains of function calls.
///
/// This is a rough 'open' version of what
/// [`StringExt`](./trait.StringExt.html) and [`VecExt`](./trait.VecExt.html)
/// do, but is made for situations where theres no extension trait.
pub trait Wrap: Sized {
    /// Turns a self reference function call into an 'inline'/'builder' call.
    ///
    /// This function normally isn't needed, if you need to access a value
    /// you can use [`Peep`](./trait.Peep.html).
    /// Theres normally something wrong with the library if you need to use
    /// this.
    fn wrap_ref<F>(self, wrap: F) -> Self
    where
        F: FnOnce(&Self),
    {
        wrap(&self);

        self
    }

    /// Turns a self mutable reference function call into an 'inline'/'builder'
    /// call.
    ///
    /// # Examples
    ///
    /// If you didn't want to use [`VecExt`](./trait.VecExt.html) for some
    /// reason you could do this.
    ///
    /// ```
    /// use fenn::Wrap;
    ///
    /// let ex = vec![1, 1, 3, 5, 5, 5, 7].wrap_mut(Vec::dedup);
    ///
    /// assert_eq!(vec![1, 3, 5, 7], ex);
    /// ```
    fn wrap_mut<F>(mut self, wrap: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        wrap(&mut self);

        self
    }

    /// Turns a consuming function call into an 'inline'/'builder' call.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::Wrap;
    ///
    /// let ex = 4.wrap_map(|n| n * 2);
    ///
    /// assert_eq!(8, ex);
    /// ```
    fn wrap_map<F, R>(self, wrap: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        wrap(self)
    }
}

impl<T> Wrap for T {}
