//! Extension traits for [`Iterator`] and [`IntoIterator`].
//!
//! [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
//! [`IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html

/// Extensions to [`IntoIterator`] that allow for conditionally chaining other
/// [`IntoIterator`]'s.
///
/// [`IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
pub trait IntoIteratorExt: IntoIterator + Sized {
    /// Conditionally chains a given iterator.
    ///
    /// # Example
    ///
    /// ```
    /// use fenn::iter::IntoIteratorExt;
    ///
    /// let vec1 = vec![1, 2, 3];
    /// let vec2 = vec![4, 5, 6];
    ///
    /// let new_vec = vec1.chain_if(true, vec2).collect::<Vec<_>>();
    ///
    /// assert_eq!(vec![1, 2, 3, 4, 5, 6], new_vec);
    /// ```
    ///
    /// ```
    /// use fenn::iter::IntoIteratorExt;
    ///
    /// let vec1 = vec![1, 2, 3];
    /// let vec2 = vec![4, 5, 6];
    ///
    /// let new_vec = vec1.chain_if(false, vec2).collect::<Vec<_>>();
    ///
    /// assert_eq!(vec![1, 2, 3], new_vec);
    /// ```
    fn chain_if<C>(self, cond: bool, iter: C) -> ChainIf<Self, C>
    where
        C: IntoIterator<Item = Self::Item>,
    {
        self.chain_if_with(cond, || iter)
    }

    /// Conditionally chains a given iterator falling back on another.
    ///
    /// # Example
    ///
    /// ```
    /// use fenn::iter::IntoIteratorExt;
    ///
    /// let vec1 = vec![1, 2, 3];
    /// let vec2 = vec![4, 5, 6];
    /// let vec3 = vec![7, 8, 9];
    ///
    /// let new_vec = vec1.chain_if_else(true, vec2, vec3).collect::<Vec<_>>();
    ///
    /// assert_eq!(vec![1, 2, 3, 4, 5, 6], new_vec);
    /// ```
    ///
    /// ```
    /// use fenn::iter::IntoIteratorExt;
    ///
    /// let vec1 = vec![1, 2, 3];
    /// let vec2 = vec![4, 5, 6];
    /// let vec3 = vec![7, 8, 9];
    ///
    /// let new_vec = vec1.chain_if_else(false, vec2, vec3).collect::<Vec<_>>();
    ///
    /// assert_eq!(vec![1, 2, 3, 7, 8, 9], new_vec);
    /// ```
    fn chain_if_else<I, E>(self, cond: bool, iter_if: I, iter_else: E) -> ChainIfElse<Self, I, E>
    where
        I: IntoIterator<Item = Self::Item>,
        E: IntoIterator<Item = Self::Item>,
    {
        self.chain_if_else_with(cond, || iter_if, || iter_else)
    }

    /// Conditionally chains a given iterator.
    ///
    /// Like `IteratorExt::chain_if`, but instead accept functions
    /// that return the iterator.
    ///
    /// # Example
    ///
    /// ```
    /// use fenn::iter::IntoIteratorExt;
    ///
    /// let vec1 = vec![1, 2, 3];
    /// let vec2 = vec![4, 5, 6];
    ///
    /// let new_vec = vec1.chain_if_with(true, || vec2).collect::<Vec<_>>();
    ///
    /// assert_eq!(vec![1, 2, 3, 4, 5, 6], new_vec);
    /// ```
    ///
    /// ```
    /// use fenn::iter::IntoIteratorExt;
    ///
    /// let vec1 = vec![1, 2, 3];
    /// let vec2 = vec![4, 5, 6];
    ///
    /// let new_vec = vec1.chain_if_with(false, || vec2).collect::<Vec<_>>();
    ///
    /// assert_eq!(vec![1, 2, 3], new_vec);
    /// ```
    fn chain_if_with<F, C>(self, cond: bool, fun: F) -> ChainIf<Self, C>
    where
        F: FnOnce() -> C,
        C: IntoIterator<Item = Self::Item>;

    /// Conditionally chains a given iterator falling back on another.
    ///
    /// Like `IteratorExt::chain_if_else`, but instead accept functions
    /// that return the iterator.
    ///
    /// # Example
    ///
    /// ```
    /// use fenn::iter::IntoIteratorExt;
    ///
    /// let vec1 = vec![1, 2, 3];
    /// let vec2 = vec![4, 5, 6];
    /// let vec3 = vec![7, 8, 9];
    ///
    /// let new_vec = vec1.chain_if_else_with(true, || vec2, || vec3).collect::<Vec<_>>();
    ///
    /// assert_eq!(vec![1, 2, 3, 4, 5, 6], new_vec);
    /// ```
    ///
    /// ```
    /// use fenn::iter::IntoIteratorExt;
    ///
    /// let vec1 = vec![1, 2, 3];
    /// let vec2 = vec![4, 5, 6];
    /// let vec3 = vec![7, 8, 9];
    ///
    /// let new_vec = vec1.chain_if_else_with(false, || vec2, || vec3).collect::<Vec<_>>();
    ///
    /// assert_eq!(vec![1, 2, 3, 7, 8, 9], new_vec);
    /// ```
    fn chain_if_else_with<FI, I, FE, E>(
        self,
        cond: bool,
        fun_if: FI,
        fun_else: FE,
    ) -> ChainIfElse<Self, I, E>
    where
        FI: FnOnce() -> I,
        I: IntoIterator<Item = Self::Item>,
        FE: FnOnce() -> E,
        E: IntoIterator<Item = Self::Item>;
}

impl<T> IntoIteratorExt for T
where
    T: IntoIterator + Sized,
{
    fn chain_if_with<F, C>(self, cond: bool, fun: F) -> ChainIf<Self, C>
    where
        F: FnOnce() -> C,
        C: IntoIterator<Item = Self::Item>,
    {
        if cond {
            ChainIf::Chained(self.into_iter(), fun().into_iter())
        } else {
            ChainIf::Original(self.into_iter())
        }
    }

    fn chain_if_else_with<FI, I, FE, E>(
        self,
        cond: bool,
        fun_if: FI,
        fun_else: FE,
    ) -> ChainIfElse<Self, I, E>
    where
        FI: FnOnce() -> I,
        I: IntoIterator<Item = Self::Item>,
        FE: FnOnce() -> E,
        E: IntoIterator<Item = Self::Item>,
    {
        if cond {
            ChainIfElse::If(self.into_iter(), fun_if().into_iter())
        } else {
            ChainIfElse::Else(self.into_iter(), fun_else().into_iter())
        }
    }
}

/// An iterator that conditionally chains together other iterators.
///
/// This `enum` is created by the [`IteratorExt::chain_if`] and
/// [`IteratorExt::chain_if_with`] functions.
///
/// [`IteratorExt::chain_if`]: ./trait.IteratorExt.html#method.chain_if
/// [`IteratorExt::chain_if_with`]: ./trait.IteratorExt.html#method.chain_if_with
pub enum ChainIf<O, C>
where
    O: IntoIterator,
    C: IntoIterator<Item = O::Item>,
{
    Chained(O::IntoIter, C::IntoIter),
    Original(O::IntoIter),
}

impl<O, C> Iterator for ChainIf<O, C>
where
    O: IntoIterator,
    C: IntoIterator<Item = O::Item>,
{
    type Item = O::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ChainIf::Chained(original, chained) => original.next().or_else(|| chained.next()),
            ChainIf::Original(original) => original.next(),
        }
    }
}

/// An iterator that conditionally chains together other iterators.
///
/// This `enum` is created by the [`IteratorExt::chain_if_else`] and
/// [`IteratorExt::chain_if_else_with`] functions.
///
/// [`IteratorExt::chain_if_else`]: ./trait.IteratorExt.html#method.chain_if_else
/// [`IteratorExt::chain_if_else_with`]: ./trait.IteratorExt.html#method.chain_if_else_with
pub enum ChainIfElse<O, I, E>
where
    O: IntoIterator,
    I: IntoIterator<Item = O::Item>,
    E: IntoIterator<Item = O::Item>,
{
    If(O::IntoIter, I::IntoIter),
    Else(O::IntoIter, E::IntoIter),
}

impl<O, I, E> Iterator for ChainIfElse<O, I, E>
where
    O: IntoIterator,
    I: IntoIterator<Item = O::Item>,
    E: IntoIterator<Item = O::Item>,
{
    type Item = O::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ChainIfElse::If(original, other) => original.next().or_else(|| other.next()),
            ChainIfElse::Else(original, other) => original.next().or_else(|| other.next()),
        }
    }
}
