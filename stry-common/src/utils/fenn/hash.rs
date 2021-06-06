use core::{
    borrow::Borrow,
    hash::{BuildHasher, Hash},
};

/// Create a [`HashMap`](https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html)
/// from a list of key-value pairs.
///
/// While based off of [maplit](https://github.com/bluss/maplit), this is an
/// extended version with the ability to set the hasher and enable a `strict`
/// mode.
///
/// # Examples
///
/// ```
/// let map = fenn::hashmap! {
///     "a" => 1,
///     "b" => 2,
/// };
///
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// ```
///
/// When `strict` mode is active, a duplicate key will cause a panic.
///
/// ```should_panic
/// let map = fenn::hashmap! {
///     strict,
///     data = {
///         "a" => 1,
///         "a" => 2, // panics
///     }
/// };
/// ```
///
/// To set the default hasher, pass in a hasher expression with the `hasher`,
/// parameter.
///
/// ```
/// use std::collections::hash_map::RandomState;
///
/// let map = fenn::hashmap! {
///     hasher = RandomState::new(),
///     data = {
///         "a" => 1,
///         "b" => 2,
///     }
/// };
///
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// ```
///
/// A custom hasher and strict mode can be used at the same time, the only
/// requirement is that the hasher comes first.
///
/// ```should_panic
/// use std::collections::hash_map::RandomState;
///
/// let map = fenn::hashmap! {
///     hasher = RandomState::new(),
///     strict,
///     data = {
///         "a" => 1,
///         "a" => 2, // panics
///     }
/// };
/// ```
#[macro_export]
macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[ $($crate::hashmap!(@single $rest)),* ]));

    ( hasher = $hasher:expr, strict, data = { $( $key:expr => $value:expr, )+ } $( , )? ) => {{
        $crate::hashmap!(hasher = $hasher, strict, data = { $( $key => $value ),+ })
    }};
    ( hasher = $hasher:expr, strict, data = { $( $key:expr => $value:expr ),* } $( , )? ) => {{
        use $crate::{HashMapExt, Peep};
        let _cap = $crate::hashmap!(@count $($key),*);
        $crate::lib::HashMap::with_capacity_and_hasher(_cap, $hasher)
        $(
            .peep(|map| assert!(map.get(&$key).is_some()) )
            .inserted($key, $value)
        )*
    }};

    ( strict, data = { $( $key:expr => $value:expr, )+ } $( , )? ) => {{
        $crate::hashmap!(strict, data = { $( $key => $value ),+ })
    }};
    ( strict, data = { $( $key:expr => $value:expr ),* } $( , )? ) => {{
        use $crate::{HashMapExt, Peep};
        let _cap = $crate::hashmap!(@count $($key),*);
        $crate::lib::HashMap::with_capacity(_cap)
        $(
            .peep(|map| assert!(map.get(&$key).is_some()) )
            .inserted($key, $value)
        )*
    }};

    ( hasher = $hasher:expr, data = { $( $key:expr => $value:expr, )+ } $( , )? ) => {{
        $crate::hashmap!(hasher = $hasher, data = { $( $key => $value ),+ })
    }};
    ( hasher = $hasher:expr, data = { $( $key:expr => $value:expr ),* } $( , )? ) => {{
        use $crate::HashMapExt;
        let _cap = $crate::hashmap!(@count $($key),*);
        $crate::lib::HashMap::with_capacity_and_hasher(_cap, $hasher)
        $(
            .inserted($key, $value)
        )*
    }};

    ( $( $key:expr => $value:expr, )+ ) => {{
        $crate::hashmap!( $( $key => $value ),+ )
    }};
    ( $( $key:expr => $value:expr ),* ) => {{
        use $crate::HashMapExt;
        let _cap = $crate::hashmap!(@count $($key),*);
        $crate::lib::HashMap::with_capacity(_cap)
        $(
            .inserted($key, $value)
        )*
    }};

    () => {
        $crate::lib::HashMap::new()
    };
}

/// Extension trait that contains functions that allow for chaining of
/// [`HashMap`](https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html)
/// functions.
///
/// Before:
///
/// ```rust
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
///
/// map.insert(37, "a");
/// map.insert(38, "b");
///
/// map.remove(&37);
///
/// assert_eq!(map.get(&37), None);
/// assert_eq!(map.get(&38), Some(&"b"));
/// ```
///
/// After:
///
/// ```rust
/// use fenn::HashMapExt;
/// use std::collections::HashMap;
///
/// let map = HashMap::new()
///     .inserted(37, "a")
///     .inserted(38, "b")
///     .removed(&37);
///
/// assert_eq!(map.get(&37), None);
/// assert_eq!(map.get(&38), Some(&"b"));
/// ```
pub trait HashMapExt<K, V, S> {
    /// Clears the map, removing all key-value pairs. Keeps the allocated memory
    /// for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::HashMapExt;
    /// use std::collections::HashMap;
    ///
    /// let a = HashMap::new()
    ///     .inserted(1, "a")
    ///     .cleared();
    ///
    /// assert!(a.is_empty());
    /// ```
    fn cleared(self) -> Self;

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did have this key present, the value is updated. The key is
    /// not updated, though; this matters for types that can be `==` without
    /// being identical.
    ///
    /// # Warning
    ///
    /// Unlike the standard [`HashMap::insert`](https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html#method.insert)
    /// that this wraps, this function ignores any returned values.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::HashMapExt;
    /// use std::collections::HashMap;
    ///
    /// let map = HashMap::new()
    ///     .inserted(37, "a");
    ///
    /// assert_eq!(map[&37], "a");
    /// assert_eq!(map.is_empty(), false);
    /// ```
    fn inserted(self, k: K, v: V) -> Self
    where
        K: Eq + Hash,
        S: BuildHasher;

    /// Removes a key from the map.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    fn removed<Q>(self, k: &Q) -> Self
    where
        K: Eq + Hash + Borrow<Q>,
        S: BuildHasher,
        Q: Eq + Hash;

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(k, v)` such that `f(&k, &mut v)`
    /// returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::HashMapExt;
    /// use std::collections::HashMap;
    ///
    /// let map = (0..8).map(|x|(x, x * 10)).collect::<HashMap<i32, i32>>()
    ///     .retained(|&k, _| k % 2 == 0);
    ///
    /// assert_eq!(map.len(), 4);
    /// ```
    fn retained<F>(self, f: F) -> Self
    where
        K: Eq + Hash,
        S: BuildHasher,
        F: FnMut(&K, &mut V) -> bool;

    /// Shrinks the capacity of the map as much as possible. It will drop
    /// down as much as possible while maintaining the internal rules
    /// and possibly leaving some space in accordance with the resize policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::HashMapExt;
    /// use std::collections::HashMap;
    ///
    /// let map: HashMap<i32, i32> = HashMap::with_capacity(100)
    ///     .inserted(1, 2)
    ///     .inserted(3, 4)
    ///     .shrinked_to_fit();
    ///
    /// assert!(map.capacity() >= 2);
    /// ```
    fn shrinked_to_fit(self) -> Self
    where
        K: Eq + Hash,
        S: BuildHasher;
}

impl<K, V, S> HashMapExt<K, V, S> for super::lib::HashMap<K, V, S> {
    fn cleared(mut self) -> Self {
        self.clear();

        self
    }

    fn inserted(mut self, k: K, v: V) -> Self
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        let _ = self.insert(k, v);

        self
    }

    fn removed<Q>(mut self, k: &Q) -> Self
    where
        K: Eq + Hash + Borrow<Q>,
        S: BuildHasher,
        Q: Eq + Hash,
    {
        let _ = self.remove(k);

        self
    }

    fn retained<F>(mut self, f: F) -> Self
    where
        K: Eq + Hash,
        S: BuildHasher,
        F: FnMut(&K, &mut V) -> bool,
    {
        self.retain(f);

        self
    }

    fn shrinked_to_fit(mut self) -> Self
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        self.shrink_to_fit();

        self
    }
}
