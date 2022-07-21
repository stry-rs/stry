use {
    crate::lib::{
        fmt,
        vec::{Drain as VecDrain, IntoIter as VecIntoIter},
    },
    core::{
        borrow::Borrow,
        iter::{
            DoubleEndedIterator, ExactSizeIterator, FromIterator, FusedIterator, IntoIterator,
            Iterator, Zip,
        },
        marker::PhantomData,
        ops::{Index, IndexMut},
        slice::{Iter as SliceIter, IterMut as SliceIterMut},
    },
};

pub struct VecMap<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> VecMap<K, V> {
    /// Creates an empty `VecMap`.
    ///
    /// The map is initially created with a capacity of 0, so it will not allocate until it
    /// is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map: VecMap<&str, i32> = VecMap::new();
    /// ```
    #[inline]
    pub fn new() -> VecMap<K, V> {
        VecMap::default()
    }

    /// Creates an empty `VecMap` with the specified capacity.
    ///
    /// The map will be able to hold at least `capacity` elements without
    /// reallocating. If `capacity` is 0, the map will not allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map: VecMap<&str, i32> = VecMap::with_capacity(10);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> VecMap<K, V> {
        VecMap {
            keys: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// This number is a lower bound; the `VecMap<K, V>` might be able to hold
    /// more, but is guaranteed to be able to hold at least this many.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let map: VecMap<i32, i32> = VecMap::with_capacity(100);
    ///
    /// assert!(map.capacity() >= 100);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.keys.capacity().min(self.values.capacity())
    }

    /// An iterator visiting all keys in arbitrary order.
    /// The iterator element type is `&'a K`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for key in map.keys() {
    ///     println!("{}", key);
    /// }
    /// ```
    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys {
            iter: self.keys.iter(),
            _phantom: PhantomData,
        }
    }

    /// An iterator visiting all values in arbitrary order.
    /// The iterator element type is `&'a V`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for val in map.values() {
    ///     println!("{}", val);
    /// }
    /// ```
    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        Values {
            iter: self.values.iter(),
            _phantom: PhantomData,
        }
    }

    /// An iterator visiting all values mutably in arbitrary order.
    /// The iterator element type is `&'a mut V`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for val in map.values_mut() {
    ///     *val = *val + 10;
    /// }
    ///
    /// for val in map.values() {
    ///     println!("{}", val);
    /// }
    /// ```
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut {
            iter: self.values.iter_mut(),
            _phantom: PhantomData,
        }
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is `(&'a K, &'a V)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for (key, val) in map.iter() {
    ///     println!("key: {} val: {}", key, val);
    /// }
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            iter: self.keys.iter().zip(self.values.iter()),
        }
    }
    /// An iterator visiting all key-value pairs in arbitrary order,
    /// with mutable references to the values.
    /// The iterator element type is `(&'a K, &'a mut V)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// // Update all values
    /// for (_, val) in map.iter_mut() {
    ///     *val *= 2;
    /// }
    ///
    /// for (key, val) in &map {
    ///     println!("key: {} val: {}", key, val);
    /// }
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut {
            iter: self.keys.iter().zip(self.values.iter_mut()),
        }
    }

    /// Returns the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// assert_eq!(map.len(), 0);
    ///
    /// map.insert(1, "a");
    ///
    /// assert_eq!(map.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Returns `true` if the map contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// assert!(map.is_empty());
    ///
    /// map.insert(1, "a");
    ///
    /// assert!(!map.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the map, returning all key-value pairs as an iterator. Keeps the
    /// allocated memory for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// map.insert(1, "a");
    /// map.insert(2, "b");
    ///
    /// for (k, v) in map.drain().take(1) {
    ///     assert!(k == 1 || k == 2);
    ///     assert!(v == "a" || v == "b");
    /// }
    ///
    /// assert!(map.is_empty());
    /// ```
    #[inline]
    pub fn drain(&mut self) -> Drain<'_, K, V> {
        Drain {
            iter: self.keys.drain(..).zip(self.values.drain(..)),
        }
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(k, v)` such that `f(&k,&mut v)` returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map: VecMap<i32, i32> = (0..8).map(|x|(x, x*10)).collect();
    ///
    /// map.retain(|&k, _| k % 2 == 0);
    ///
    /// assert_eq!(map.len(), 4);
    /// ```
    #[inline]
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        for i in (0..self.len()).rev() {
            if !f(&self.keys[i], &mut self.values[i]) {
                self.keys.swap_remove(i);
                self.values.swap_remove(i);
            }
        }
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory
    /// for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// map.insert(1, "a");
    ///
    /// map.clear();
    ///
    /// assert!(map.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.keys.clear();
        self.values.clear();
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the `VecMap`. The collection may reserve more space to avoid
    /// frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new allocation size overflows [`usize`].
    ///
    /// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map: VecMap<&str, i32> = VecMap::new();
    ///
    /// map.reserve(10);
    /// ```
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.keys.reserve(additional);
        self.values.reserve(additional);
    }

    /// Shrinks the capacity of the map as much as possible. It will drop
    /// down as much as possible while maintaining the internal rules
    /// and possibly leaving some space in accordance with the resize policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map: VecMap<i32, i32> = VecMap::with_capacity(100);
    ///
    /// map.insert(1, 2);
    /// map.insert(3, 4);
    ///
    /// assert!(map.capacity() >= 100);
    ///
    /// map.shrink_to_fit();
    ///
    /// assert!(map.capacity() >= 2);
    /// ```
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.keys.shrink_to_fit();
        self.values.shrink_to_fit();
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.keys.truncate(len);
        self.values.truncate(len);
    }
}

impl<K, V> VecMap<K, V>
where
    K: PartialEq,
{
    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut letters = VecMap::new();
    ///
    /// for ch in "a short treatise on fungi".chars() {
    ///     let counter = letters.entry(ch).or_insert(0);
    ///
    ///     *counter += 1;
    /// }
    ///
    /// assert_eq!(letters[&'s'], 2);
    /// assert_eq!(letters[&'t'], 3);
    /// assert_eq!(letters[&'u'], 1);
    /// assert_eq!(letters.get(&'y'), None);
    /// ```
    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        match self.iter_mut().position(|(k, _)| &key == k) {
            Some(index) => Entry::Occupied(OccupiedEntry { index, map: self }),
            None => Entry::Vacant(VacantEntry { key, map: self }),
        }
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`PartialEq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// map.insert(1, "a");
    ///
    /// assert_eq!(map.get(&1), Some(&"a"));
    /// assert_eq!(map.get(&2), None);
    /// ```
    #[inline]
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&'_ V>
    where
        K: Borrow<Q>,
        Q: PartialEq<K>,
    {
        self.keys
            .iter()
            .position(|k| key == k)
            .map(|p| &self.values[p])
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`PartialEq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// map.insert(1, "a");
    ///
    /// assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
    /// assert_eq!(map.get_key_value(&2), None);
    /// ```
    #[inline]
    pub fn get_key_value<Q: ?Sized>(&self, key: &Q) -> Option<(&'_ K, &'_ V)>
    where
        K: Borrow<Q>,
        Q: PartialEq<K>,
    {
        self.keys
            .iter()
            .position(|k| key == k)
            .map(|p| (&self.keys[p], &self.values[p]))
    }

    /// Returns the key-value pair corresponding to the supplied key, with a mutable reference to value.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`PartialEq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// map.insert(1, "a");
    ///
    /// let (k, v) = map.get_key_value_mut(&1).unwrap();
    ///
    /// assert_eq!(k, &1);
    /// assert_eq!(v, &mut "a");
    ///
    /// *v = "b";
    ///
    /// assert_eq!(map.get_key_value_mut(&1), Some((&1, &mut "b")));
    /// assert_eq!(map.get_key_value_mut(&2), None);
    /// ```
    #[inline]
    pub fn get_key_value_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<(&'_ K, &'_ mut V)>
    where
        K: Borrow<Q>,
        Q: PartialEq<K>,
    {
        self.keys
            .iter_mut()
            .position(|k| key == k)
            .map(move |p| (&self.keys[p], &mut self.values[p]))
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`PartialEq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// map.insert(1, "a");
    ///
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    #[inline]
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: PartialEq<K>,
    {
        self.keys.iter().any(|k| key == k)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`PartialEq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// map.insert(1, "a");
    ///
    /// if let Some(x) = map.get_mut(&1) {
    ///     *x = "b";
    /// }
    ///
    /// assert_eq!(map[&1], "b");
    /// ```
    #[inline]
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&'_ mut V>
    where
        K: Borrow<Q>,
        Q: PartialEq<K>,
    {
        self.keys
            .iter()
            .position(|k| key == k)
            .map(move |p| &mut self.values[p])
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// assert_eq!(map.insert(37, "a"), None);
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert(37, "b");
    ///
    /// assert_eq!(map.insert(37, "c"), Some("b"));
    /// assert_eq!(map[&37], "c");
    /// ```
    #[inline]
    pub fn insert(&mut self, key: K, mut value: V) -> Option<V> {
        if let Some(position) = self.keys.iter().position(|k| &key == k) {
            core::mem::swap(&mut value, &mut self.values[position]);

            Some(value)
        } else {
            self.keys.push(key);
            self.values.push(value);

            None
        }
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`PartialEq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// map.insert(1, "a");
    ///
    /// assert_eq!(map.remove(&1), Some("a"));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    #[inline]
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: PartialEq<K>,
    {
        if let Some(index) = self.keys.iter().position(|k| key == k) {
            self.keys.swap_remove(index);

            Some(self.values.swap_remove(index))
        } else {
            None
        }
    }

    /// Removes a key from the map, returning the stored key and value if the
    /// key was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`PartialEq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
    ///
    /// # Examples
    ///
    /// ```
    /// use fenn::vec_map::VecMap;
    ///
    /// let mut map = VecMap::new();
    ///
    /// map.insert(1, "a");
    ///
    /// assert_eq!(map.remove_entry(&1), Some((1, "a")));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    #[inline]
    pub fn remove_entry<Q: ?Sized>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: PartialEq<K>,
    {
        if let Some(index) = self.keys.iter().position(|k| key == k) {
            Some((self.keys.swap_remove(index), self.values.swap_remove(index)))
        } else {
            None
        }
    }
}

impl<'a, K, V> IntoIterator for &'a VecMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut VecMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V> IntoIterator for VecMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.keys.into_iter().zip(self.values.into_iter()),
        }
    }
}

impl<K, V> FromIterator<(K, V)> for VecMap<K, V>
where
    K: PartialEq,
{
    #[inline]
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let iter = iter.into_iter();

        let mut map = Self::with_capacity(iter.size_hint().0);

        iter.for_each(|(k, v)| {
            map.insert(k, v);
        });

        map
    }
}

impl<K, Q: ?Sized, V> Index<&Q> for VecMap<K, V>
where
    K: PartialEq + Borrow<Q>,
    Q: PartialEq<K>,
{
    type Output = V;

    #[inline]
    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

impl<K, Q: ?Sized, V> IndexMut<&Q> for VecMap<K, V>
where
    K: PartialEq + Borrow<Q>,
    Q: PartialEq<K>,
{
    #[inline]
    fn index_mut(&mut self, key: &Q) -> &mut V {
        self.get_mut(key).expect("no entry found for key")
    }
}

impl<K, V> Clone for VecMap<K, V>
where
    K: Clone,
    V: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        VecMap {
            keys: self.keys.clone(),
            values: self.values.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        self.keys.clone_from(&source.keys);
        self.values.clone_from(&source.values);
    }
}

impl<K, V> fmt::Debug for VecMap<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V> Default for VecMap<K, V> {
    #[inline]
    fn default() -> Self {
        Self::with_capacity(0)
    }
}

impl<K, V> PartialEq<VecMap<K, V>> for VecMap<K, V>
where
    K: PartialEq,
    V: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.keys.eq(&other.keys) && self.values.eq(&other.values)
    }
}

impl<K, V> Eq for VecMap<K, V>
where
    K: Eq,
    V: Eq,
{
}

pub struct Keys<'a, K: 'a, V> {
    iter: SliceIter<'a, K>,
    _phantom: PhantomData<V>,
}

impl<'a, K, V> Clone for Keys<'a, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Keys<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for Keys<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

pub struct Values<'a, K, V: 'a> {
    iter: SliceIter<'a, V>,
    _phantom: PhantomData<K>,
}

impl<'a, K, V> Clone for Values<'a, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<K, V> fmt::Debug for Values<'_, K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter.fmt(f)
    }
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Values<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for Values<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> FusedIterator for Values<'a, K, V> {}

pub struct ValuesMut<'a, K, V: 'a> {
    iter: SliceIterMut<'a, V>,
    _phantom: PhantomData<K>,
}

impl<K, V> fmt::Debug for ValuesMut<'_, K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter.fmt(f)
    }
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for ValuesMut<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for ValuesMut<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> FusedIterator for ValuesMut<'a, K, V> {}

pub struct Iter<'a, K: 'a, V: 'a> {
    iter: Zip<SliceIter<'a, K>, SliceIter<'a, V>>,
}

impl<K, V> fmt::Debug for Iter<'_, K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter.fmt(f)
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Iter<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> FusedIterator for Iter<'a, K, V> {}

pub struct IterMut<'a, K: 'a, V: 'a> {
    iter: Zip<SliceIter<'a, K>, SliceIterMut<'a, V>>,
}

impl<K, V> fmt::Debug for IterMut<'_, K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter.fmt(f)
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for IterMut<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for IterMut<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> FusedIterator for IterMut<'a, K, V> {}

pub struct IntoIter<K, V> {
    iter: Zip<VecIntoIter<K>, VecIntoIter<V>>,
}

impl<K, V> fmt::Debug for IntoIter<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter.fmt(f)
    }
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<K, V> DoubleEndedIterator for IntoIter<K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<K, V> ExactSizeIterator for IntoIter<K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V> FusedIterator for IntoIter<K, V> {}

pub struct Drain<'a, K: 'a, V: 'a> {
    iter: Zip<VecDrain<'a, K>, VecDrain<'a, V>>,
}

impl<K, V> fmt::Debug for Drain<'_, K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter.fmt(f)
    }
}

impl<'a, K, V> Iterator for Drain<'a, K, V> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Drain<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for Drain<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> FusedIterator for Drain<'a, K, V> {}

pub enum Entry<'a, K: 'a, V: 'a> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K: 'a, V: 'a> Entry<'a, K, V> {
    #[inline]
    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(entry) => &entry.map.keys[entry.index],
            Entry::Vacant(entry) => entry.key(),
        }
    }

    #[inline]
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());

                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}

impl<'a, K: 'a, V: 'a> Entry<'a, K, V>
where
    K: PartialEq,
{
    #[inline]
    pub fn insert(self, value: V) -> OccupiedEntry<'a, K, V> {
        match self {
            Entry::Occupied(mut entry) => {
                entry.insert(value);

                entry
            }
            Entry::Vacant(entry) => {
                entry.map.insert(entry.key, value);

                OccupiedEntry {
                    index: entry.map.values.len() - 1,
                    map: entry.map,
                }
            }
        }
    }

    #[inline]
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }
}

impl<'a, K: 'a, V: 'a> Entry<'a, K, V>
where
    K: PartialEq,
    V: Default,
{
    #[inline]
    pub fn or_default(self) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(Default::default()),
        }
    }
}

impl<K, V> fmt::Debug for Entry<'_, K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Entry::Occupied(ref e) => f.debug_tuple("Entry").field(e).finish(),
            Entry::Vacant(ref e) => f.debug_tuple("Entry").field(e).finish(),
        }
    }
}

pub struct OccupiedEntry<'a, K: 'a, V: 'a> {
    index: usize,
    map: &'a mut VecMap<K, V>,
}

impl<'a, K: 'a, V: 'a> OccupiedEntry<'a, K, V> {
    #[inline]
    pub fn key(&self) -> &K {
        &self.map.keys[self.index]
    }

    #[inline]
    pub fn get(&self) -> &V {
        &self.map.values[self.index]
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut V {
        &mut self.map.values[self.index]
    }

    #[inline]
    pub fn into_mut(self) -> &'a mut V {
        &mut self.map.values[self.index]
    }

    #[inline]
    pub fn insert(&mut self, mut value: V) -> V {
        core::mem::swap(&mut value, &mut self.map.values[self.index]);

        value
    }
}

impl<'a, K: 'a, V: 'a> OccupiedEntry<'a, K, V>
where
    K: PartialEq,
{
    #[inline]
    pub fn remove(self) -> V {
        self.map.keys.swap_remove(self.index);

        self.map.values.swap_remove(self.index)
    }

    #[inline]
    pub fn remove_entry(self) -> (K, V) {
        (
            self.map.keys.swap_remove(self.index),
            self.map.values.swap_remove(self.index),
        )
    }
}

impl<K, V> fmt::Debug for OccupiedEntry<'_, K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OccupiedEntry")
            .field("index", &self.index)
            .field("value", &self.map.values[self.index])
            .finish()
    }
}

pub struct VacantEntry<'a, K: 'a, V: 'a> {
    key: K,
    map: &'a mut VecMap<K, V>,
}

impl<'a, K: 'a, V: 'a> VacantEntry<'a, K, V> {
    #[inline]
    pub fn key(&self) -> &K {
        &self.key
    }

    #[inline]
    pub fn into_key(self) -> K {
        self.key
    }
}

impl<'a, K: 'a, V: 'a> VacantEntry<'a, K, V>
where
    K: PartialEq,
{
    #[inline]
    pub fn insert(self, value: V) -> &'a mut V {
        self.map.insert(self.key, value);

        let index = self.map.values.len() - 1;

        self.map
            .values
            .get_mut(index)
            .expect("no entry found for key")
    }
}

impl<K, V> fmt::Debug for VacantEntry<'_, K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VacantEntry")
            .field("key", &self.key)
            .finish()
    }
}
