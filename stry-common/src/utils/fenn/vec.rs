use core::cmp::Ordering;

/// Extension trait that contains functions that allow for chaining of
/// [`Vec`].
///
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
///
/// Before:
///
/// ```rust
/// let mut vec = vec![2, 4, 3, 1, 5, 2, 3, 1];
///
/// vec.sort();
///
/// vec.dedup();
///
/// assert_eq!(vec, [1, 2, 3, 4, 5]);
/// ```
///
/// After:
///
/// ```rust
/// use fenn::VecExt;
///
/// let vec = vec![2, 4, 3, 1, 5, 2, 3, 1]
///   .sorted()
///   .deduped();
///
/// assert_eq!(vec, [1, 2, 3, 4, 5]);
/// ```
pub trait VecExt<T> {
    /// Moves all the elements of `other` into `Self`, leaving `other` empty.
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in the vector overflows a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// let mut vec2 = vec![4, 5, 6];
    /// let vec = vec![1, 2, 3].appended(&mut vec2);
    ///
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
    /// assert_eq!(vec2, []);
    /// ```
    fn appended(self, other: &mut Self) -> Self;

    /// Clears the vector, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// let v = vec![1, 2, 3].cleared();
    ///
    /// assert!(v.is_empty());
    /// ```
    fn cleared(self) -> Self;

    /// Removes consecutive repeated elements in the vector according to the
    /// [`PartialEq`] trait implementation.
    ///
    /// If the vector is sorted, this removes all duplicates.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// let vec = vec![1, 2, 2, 3, 2].deduped();
    ///
    /// assert_eq!(vec, [1, 2, 3, 2]);
    /// ```
    fn deduped(self) -> Self
    where
        T: PartialEq;

    /// Removes all but the first of consecutive elements in the vector satisfying a given equality
    /// relation.
    ///
    /// The `same_bucket` function is passed references to two elements from the vector and
    /// must determine if the elements compare equal. The elements are passed in opposite order
    /// from their order in the vector, so if `same_bucket(a, b)` returns `true`, `a` is removed.
    ///
    /// If the vector is sorted, this removes all duplicates.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// let vec = vec!["foo", "bar", "Bar", "baz", "bar"]
    ///     .deduped_by(|a, b| a.eq_ignore_ascii_case(b));
    ///
    /// assert_eq!(vec, ["foo", "bar", "baz", "bar"]);
    /// ```
    fn deduped_by<F>(self, same_bucket: F) -> Self
    where
        F: FnMut(&mut T, &mut T) -> bool;

    /// Removes all but the first of consecutive elements in the vector that resolve to the same
    /// key.
    ///
    /// If the vector is sorted, this removes all duplicates.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// let vec = vec![10, 20, 21, 30, 20].deduped_by_key(|i| *i / 10);
    ///
    /// assert_eq!(vec, [10, 20, 30, 20]);
    /// ```
    fn deduped_by_key<F, K>(self, key: F) -> Self
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq<K>;

    /// Resizes the `Vec` in-place so that `len` is equal to `new_len`.
    ///
    /// If `new_len` is greater than `len`, the `Vec` is extended by the
    /// difference, with each additional slot filled with `value`.
    /// If `new_len` is less than `len`, the `Vec` is simply truncated.
    ///
    /// This method requires `T` to implement [`Clone`],
    /// in order to be able to clone the passed value.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// let vec = vec!["hello"].resized(3, "world");
    ///
    /// assert_eq!(vec, ["hello", "world", "world"]);
    /// ```
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// let vec = vec![1, 2, 3, 4].resized(2, 0);
    ///
    /// assert_eq!(vec, [1, 2]);
    /// ```
    ///
    /// [`Clone`]: ../../std/clone/trait.Clone.html
    fn resized(self, new_len: usize, value: T) -> Self
    where
        T: Clone;

    /// Reverses the order of elements in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// let v = vec![1, 2, 3].reversed();
    ///
    /// assert!(v == [3, 2, 1]);
    /// ```
    fn reversed(self) -> Self;

    /// Shrinks the capacity of the vector as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the vector that there is space for a few more elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::{Peep, VecExt};
    ///
    /// let mut vec2 = vec![1, 2, 3];
    /// let vec = Vec::with_capacity(10)
    ///     .appended(&mut vec2)
    ///     .peep(|vec| assert_eq!(vec.capacity(), 10))
    ///     .shrinked_to_fit();
    ///
    /// assert!(vec.capacity() >= 3);
    /// assert_eq!(vec2, []);
    /// ```
    fn shrinked_to_fit(self) -> Self;

    /// Sorts the vector.
    ///
    /// This sort is stable (i.e., does not reorder equal elements) and `O(n * log(n))` worst-case.
    ///
    /// # Current implementation
    ///
    /// The current algorithm is an adaptive, iterative merge sort inspired by
    /// [timsort](https://en.wikipedia.org/wiki/Timsort).
    /// It is designed to be very fast in cases where the vector is nearly sorted, or consists of
    /// two or more sorted sequences concatenated one after another.
    ///
    /// Also, it allocates temporary storage half the size of `self`, but for short vectors a
    /// non-allocating insertion sort is used instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// let v = vec![-5, 4, 1, -3, 2].sorted();
    ///
    /// assert!(v == [-5, -3, 1, 2, 4]);
    /// ```
    fn sorted(self) -> Self
    where
        T: Ord;

    /// Sorts the vector with a comparator function.
    ///
    /// This sort is stable (i.e., does not reorder equal elements) and `O(n * log(n))` worst-case.
    ///
    /// The comparator function must define a total ordering for the elements in the vector. If
    /// the ordering is not total, the order of the elements is unspecified. An order is a
    /// total order if it is (for all `a`, `b` and `c`):
    ///
    /// * total and antisymmetric: exactly one of `a < b`, `a == b` or `a > b` is true, and
    /// * transitive, `a < b` and `b < c` implies `a < c`. The same must hold for both `==` and `>`.
    ///
    /// For example, while [`f64`] doesn't implement [`Ord`] because `NaN != NaN`, we can use
    /// `partial_cmp` as our sort function when we know the vector doesn't contain a `NaN`.
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// let mut floats = vec![5f64, 4.0, 1.0, 3.0, 2.0]
    ///     .sorted_by(|a, b| a.partial_cmp(b).unwrap());
    ///
    /// assert_eq!(floats, [1.0, 2.0, 3.0, 4.0, 5.0]);
    /// ```
    ///
    /// # Current implementation
    ///
    /// The current algorithm is an adaptive, iterative merge sort inspired by
    /// [timsort](https://en.wikipedia.org/wiki/Timsort).
    /// It is designed to be very fast in cases where the vector is nearly sorted, or consists of
    /// two or more sorted sequences concatenated one after another.
    ///
    /// Also, it allocates temporary storage half the size of `self`, but for short vectors a
    /// non-allocating insertion sort is used instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// let mut v = vec![5, 4, 1, 3, 2]
    ///     .sorted_by(|a, b| a.cmp(b));
    ///
    /// assert!(v == [1, 2, 3, 4, 5]);
    /// ```
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// // reverse sorting
    /// let v = vec![1, 2, 3, 4, 5]
    ///     .sorted_by(|a, b| b.cmp(a));
    ///
    /// assert!(v == [5, 4, 3, 2, 1]);
    /// ```
    fn sorted_by<F>(self, compare: F) -> Self
    where
        F: FnMut(&T, &T) -> Ordering;

    /// Sorts the vector with a key extraction function.
    ///
    /// This sort is stable (i.e., does not reorder equal elements) and `O(m * n * log(n))`
    /// worst-case, where the key function is `O(m)`.
    ///
    /// # Current implementation
    ///
    /// The current algorithm is an adaptive, iterative merge sort inspired by
    /// [timsort](https://en.wikipedia.org/wiki/Timsort).
    /// It is designed to be very fast in cases where the vector is nearly sorted, or consists of
    /// two or more sorted sequences concatenated one after another.
    ///
    /// Also, it allocates temporary storage half the size of `self`, but for short vectors a
    /// non-allocating insertion sort is used instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// let v = vec![-5i32, 4, 1, -3, 2]
    ///     .sorted_by_key(|k| k.abs());
    ///
    /// assert!(v == [1, 2, -3, 4, -5]);
    /// ```
    fn sorted_by_key<F, K>(self, f: F) -> Self
    where
        F: FnMut(&T) -> K,
        K: Ord;

    /// Shortens the vector, keeping the first `len` elements and dropping
    /// the rest.
    ///
    /// If `len` is greater than the vector's current length, this has no
    /// effect.
    ///
    /// The `drain` method can emulate `truncate`, but causes the excess
    /// elements to be returned instead of dropped.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the vector.
    ///
    /// # Examples
    ///
    /// Truncating a five element vector to two elements:
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// let vec = vec![1, 2, 3, 4, 5].truncated(2);
    ///
    /// assert_eq!(vec, [1, 2]);
    /// ```
    ///
    /// No truncation occurs when `len` is greater than the vector's current
    /// length:
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// let vec = vec![1, 2, 3].truncated(8);
    ///
    /// assert_eq!(vec, [1, 2, 3]);
    /// ```
    ///
    /// Truncating when `len == 0` is equivalent to calling the [`cleared`]
    /// method.
    ///
    /// ```
    /// # extern crate fenn;
    /// use fenn::VecExt;
    ///
    /// let vec = vec![1, 2, 3].truncated(0);
    ///
    /// assert_eq!(vec, []);
    /// ```
    ///
    /// [`cleared`]: #tymethod.cleared
    fn truncated(self, len: usize) -> Self;
}

impl<T> VecExt<T> for super::lib::vec::Vec<T> {
    fn appended(mut self, other: &mut Self) -> Self {
        self.append(other);

        self
    }

    fn cleared(mut self) -> Self {
        self.clear();

        self
    }

    fn deduped(mut self) -> Self
    where
        T: PartialEq,
    {
        self.dedup();

        self
    }

    fn deduped_by<F>(mut self, same_bucket: F) -> Self
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        self.dedup_by(same_bucket);

        self
    }

    fn deduped_by_key<F, K>(mut self, key: F) -> Self
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq<K>,
    {
        self.dedup_by_key(key);

        self
    }

    fn resized(mut self, new_len: usize, value: T) -> Self
    where
        T: Clone,
    {
        self.resize(new_len, value);

        self
    }

    fn reversed(mut self) -> Self {
        self.reverse();

        self
    }

    fn shrinked_to_fit(mut self) -> Self {
        self.shrink_to_fit();

        self
    }

    fn sorted(mut self) -> Self
    where
        T: Ord,
    {
        self.sort();

        self
    }

    fn sorted_by<F>(mut self, compare: F) -> Self
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.sort_by(compare);

        self
    }

    fn sorted_by_key<F, K>(mut self, f: F) -> Self
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.sort_by_key(f);

        self
    }

    fn truncated(mut self, len: usize) -> Self {
        self.truncate(len);

        self
    }
}
