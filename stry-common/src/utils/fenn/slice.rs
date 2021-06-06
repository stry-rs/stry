//! Types allowing for the 'lazy' slicing of `&str`s while keeping a single lifetime.

use std::{
    fmt::{Display, Error, Formatter},
    ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

/// A 'lazy' [`str`] slice using [`Range`]s.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Slice<'s> {
    source: &'s str,
    range: Range<usize>,
}

impl<'s> Slice<'s> {
    pub const fn new(source: &'s str) -> Slice<'s> {
        Slice {
            range: 0..source.len(),
            source,
        }
    }

    /// Returns the length of `self`.
    ///
    /// This length is in bytes, not [`char`]s or graphemes. In other words,
    /// it may not be what a human considers the length of the string.
    ///
    /// [`char`]: prim@char
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use lazy_slice::Slice;
    /// let len = Slice::new("foo").len();
    /// assert_eq!(3, len);
    ///
    /// assert_eq!(Slice::new("ƒoo").len(), 4); // fancy f!
    /// # // assert_eq!(Slice::new("ƒoo").chars().count(), 3);
    /// ```
    pub const fn len(&self) -> usize {
        self.range.end - self.range.start
    }

    /// Returns `true` if `self` has a length of zero bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use lazy_slice::Slice;
    /// let s = Slice::new("");
    /// assert!(s.is_empty());
    ///
    /// let s = Slice::new("not empty");
    /// assert!(!s.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.range.start == self.range.end
    }

    //

    /// Returns `true` if the given pattern matches a prefix of this
    /// string slice.
    ///
    /// Returns `false` if it does not.
    ///
    /// The [pattern] can be a `&str`, [`char`], a slice of [`char`]s, or a
    /// function or closure that determines if a character matches.
    ///
    /// [`char`]: prim@char
    /// [pattern]: std::str::pattern::Pattern
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use lazy_slice::Slice;
    /// let bananas = Slice::new("bananas");
    ///
    /// assert!(bananas.starts_with("bana"));
    /// assert!(!bananas.starts_with("nana"));
    /// ```
    pub fn starts_with<'r, P, F>(&self, pat: P) -> bool
    where
        P: Into<Pattern<'r, F>>,
        F: FnMut(char) -> bool,
    {
        let pat: Pattern<'r, F> = pat.into();

        let slice = &self.source[self.range.start..self.range.end];

        match pat {
            Pattern::Char(pat) => slice.starts_with(pat),
            Pattern::CharArrayRef(pat) => slice.starts_with(pat),
            Pattern::Function(pat) => slice.starts_with(pat),
            Pattern::Str(pat) => slice.starts_with(pat),
            Pattern::StrRef(pat) => slice.starts_with(pat),
            Pattern::StringRef(pat) => slice.starts_with(pat),
        }
    }

    /// Returns `true` if the given pattern matches a suffix of this
    /// string slice.
    ///
    /// Returns `false` if it does not.
    ///
    /// The [pattern] can be a `&str`, [`char`], a slice of [`char`]s, or a
    /// function or closure that determines if a character matches.
    ///
    /// [`char`]: prim@char
    /// [pattern]: std::str::pattern::Pattern
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use lazy_slice::Slice;
    /// let bananas = Slice::new("bananas");
    ///
    /// assert!(bananas.ends_with("anas"));
    /// assert!(!bananas.ends_with("nana"));
    /// ```
    pub fn ends_with<'r, P, F>(&self, pat: P) -> bool
    where
        P: Into<Pattern<'r, F>>,
        F: FnMut(char) -> bool,
    {
        let pat: Pattern<'r, F> = pat.into();

        let slice = &self.source[self.range.start..self.range.end];

        match pat {
            Pattern::Char(pat) => slice.ends_with(pat),
            Pattern::CharArrayRef(pat) => slice.ends_with(pat),
            Pattern::Function(pat) => slice.ends_with(pat),
            Pattern::Str(pat) => slice.ends_with(pat),
            Pattern::StrRef(pat) => slice.ends_with(pat),
            Pattern::StringRef(pat) => slice.ends_with(pat),
        }
    }

    /// Returns `true` if the given pattern matches a sub-slice of
    /// this string slice.
    ///
    /// Returns `false` if it does not.
    ///
    /// The [pattern] can be a `&str`, [`char`], a slice of [`char`]s, or a
    /// function or closure that determines if a character matches.
    ///
    /// [`char`]: prim@char
    /// [pattern]: std::str::pattern::Pattern
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use lazy_slice::Slice;
    /// let bananas = Slice::new("bananas");
    ///
    /// assert!(bananas.contains("nana"));
    /// assert!(!bananas.contains("apples"));
    /// ```
    pub fn contains<'r, P, F>(&self, pat: P) -> bool
    where
        P: Into<Pattern<'r, F>>,
        F: FnMut(char) -> bool,
    {
        let pat: Pattern<'r, F> = pat.into();

        let slice = &self.source[self.range.start..self.range.end];

        match pat {
            Pattern::Char(pat) => slice.contains(pat),
            Pattern::CharArrayRef(pat) => slice.contains(pat),
            Pattern::Function(pat) => slice.contains(pat),
            Pattern::Str(pat) => slice.contains(pat),
            Pattern::StrRef(pat) => slice.contains(pat),
            Pattern::StringRef(pat) => slice.contains(pat),
        }
    }

    //

    // pub fn lines(&self) -> Lines<'s> {
    //     self.split('\n');

    //     todo!()
    // }

    //

    // pub fn split<'r, P, F>(&self, pat: P)
    // where
    //     P: Into<Pattern<'r, F>>,
    //     F: FnMut(char) -> bool,
    // {
    // }

    //

    /// Returns a string slice with leading and trailing whitespace removed.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived
    /// Core Property `White_Space`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use lazy_slice::Slice;
    /// let s = Slice::new(" Hello\tworld\t");
    ///
    /// assert_eq!("Hello\tworld", s.trim().slice());
    /// ```
    pub fn trim(&self) -> Slice<'s> {
        self.trim_start().trim_end()
    }

    /// Returns a string slice with leading whitespace removed.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived
    /// Core Property `White_Space`.
    ///
    /// # Text directionality
    ///
    /// A string is a sequence of bytes. `start` in this context means the first
    /// position of that byte string; for a left-to-right language like English or
    /// Russian, this will be left side, and for right-to-left languages like
    /// Arabic or Hebrew, this will be the right side.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use lazy_slice::Slice;
    /// let s = Slice::new(" Hello\tworld\t");
    /// assert_eq!("Hello\tworld\t", s.trim_start().slice());
    /// ```
    ///
    /// Directionality:
    ///
    /// ```
    /// # use lazy_slice::Slice;
    /// let s = Slice::new("  English  ");
    /// assert!(Some('E') == s.trim_start().slice().chars().next());
    ///
    /// let s = Slice::new("  עברית  ");
    /// assert!(Some('ע') == s.trim_start().slice().chars().next());
    /// ```
    pub fn trim_start(&self) -> Slice<'s> {
        let slice = &self.source[self.range.start..self.range.end];

        if !slice.starts_with(|c: char| c.is_whitespace()) {
            // return early if no whitespace
            return Slice {
                source: self.source,
                range: self.range.start..self.range.end,
            };
        }

        let mut up_to = 0;

        for (i, c) in slice.char_indices() {
            if !c.is_whitespace() {
                break;
            }

            up_to = i;
        }

        // last index points to the start of the last whitespace
        // we need to remove it
        up_to += 1;

        Slice {
            source: self.source,
            range: (self.range.start + up_to)..self.range.end,
        }
    }

    /// Returns a string slice with trailing whitespace removed.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived
    /// Core Property `White_Space`.
    ///
    /// # Text directionality
    ///
    /// A string is a sequence of bytes. `end` in this context means the last
    /// position of that byte string; for a left-to-right language like English or
    /// Russian, this will be right side, and for right-to-left languages like
    /// Arabic or Hebrew, this will be the left side.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use lazy_slice::Slice;
    /// let s = Slice::new(" Hello\tworld\t");
    /// assert_eq!(" Hello\tworld", s.trim_end().slice());
    /// ```
    ///
    /// Directionality:
    ///
    /// ```
    /// # use lazy_slice::Slice;
    /// let s = Slice::new("  English  ");
    /// assert!(Some('h') == s.trim_end().slice().chars().rev().next());
    ///
    /// let s = Slice::new("  עברית  ");
    /// assert!(Some('ת') == s.trim_end().slice().chars().rev().next());
    /// ```
    pub fn trim_end(&self) -> Slice<'s> {
        let slice = &self.source[self.range.start..self.range.end];

        if !slice.ends_with(|c: char| c.is_whitespace()) {
            // return early if no whitespace
            return Slice {
                source: self.source,
                range: self.range.start..self.range.end,
            };
        }

        let mut down_to = 0;

        for (i, c) in slice.char_indices().rev() {
            if !c.is_whitespace() {
                break;
            }

            down_to = i;
        }

        if down_to == 0 {
            // there were no whitespace
            down_to = self.range.end;
        }

        Slice {
            source: self.source,
            range: self.range.start..(down_to + self.range.start),
        }
    }

    //

    /// It isn't possible to return a owned [`Slice`] from a [`Index`], so you have to use this function.
    ///
    /// [`Index`]: std::ops::Index
    pub fn index<R>(&self, range: R) -> Slice<'s>
    where
        R: Into<Ranges>,
    {
        Slice {
            source: self.source,
            range: Self::normalize_ranges((self.range.start)..(self.range.end), range.into()),
        }
    }

    #[inline]
    const fn normalize_ranges(base: Range<usize>, range: Ranges) -> Range<usize> {
        match range {
            // ..
            Ranges::RangeFull(_) => (base.start)..(base.end),
            // <num>..<num>
            Ranges::Range(range) => Self::convert_range((base.start)..(base.end), range),
            // <num>..=<num>
            Ranges::RangeInclusive(range) => Self::convert_range(
                (base.start)..(base.end),
                (*range.start())..((*range.end()) + 1),
            ),
            // ..<num>
            Ranges::RangeTo(range) => Self::convert_range_to((base.start)..(base.end), range),
            // ..=<num>
            Ranges::RangeToInclusive(range) => {
                Self::convert_range_to((base.start)..(base.end), ..(range.end + 1))
            }
            // <num>..
            Ranges::RangeFrom(range) => (base.start + range.start)..(base.end),
        }
    }

    #[inline]
    const fn convert_range(base: Range<usize>, other: Range<usize>) -> Range<usize> {
        (base.start + other.start)..(if other.end == 0 {
            base.end
        } else {
            base.start + other.end
        })
    }

    #[inline]
    const fn convert_range_to(base: Range<usize>, other: RangeTo<usize>) -> Range<usize> {
        (base.start)..(if other.end == 0 {
            base.end
        } else {
            base.start + other.end
        })
    }

    //

    /// Consume and 'run' the slice, returning the given range of the source [`str`].
    pub fn slice(mut self) -> &'s str {
        self.source = &self.source[self.range.start..self.range.end];

        self.source
    }
}

impl<'s> Clone for Slice<'s> {
    fn clone(&self) -> Self {
        Slice {
            source: self.source,
            range: self.range.start..self.range.end,
        }
    }
}

impl<'s> Display for Slice<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", &self.source[self.range.start..self.range.end])
    }
}

// pub struct Split<'s> {
//     source: &'s str,
// }

// impl<'s> Iterator for Split<'s> {
//     type Item = Slice<'s>;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

// pub struct Lines<'s> {
//     source: &'s str,
// }

// impl<'s> Iterator for Lines<'s> {
//     type Item = Slice<'s>;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

pub enum Ranges {
    Range(Range<usize>),
    RangeFrom(RangeFrom<usize>),
    RangeFull(RangeFull),
    RangeInclusive(RangeInclusive<usize>),
    RangeTo(RangeTo<usize>),
    RangeToInclusive(RangeToInclusive<usize>),
}

impl From<Range<usize>> for Ranges {
    fn from(range: Range<usize>) -> Ranges {
        Ranges::Range(range)
    }
}

impl From<RangeFrom<usize>> for Ranges {
    fn from(range: RangeFrom<usize>) -> Ranges {
        Ranges::RangeFrom(range)
    }
}

impl From<RangeFull> for Ranges {
    fn from(range: RangeFull) -> Ranges {
        Ranges::RangeFull(range)
    }
}

impl From<RangeInclusive<usize>> for Ranges {
    fn from(range: RangeInclusive<usize>) -> Ranges {
        Ranges::RangeInclusive(range)
    }
}

impl From<RangeTo<usize>> for Ranges {
    fn from(range: RangeTo<usize>) -> Ranges {
        Ranges::RangeTo(range)
    }
}

impl From<RangeToInclusive<usize>> for Ranges {
    fn from(range: RangeToInclusive<usize>) -> Ranges {
        Ranges::RangeToInclusive(range)
    }
}

/// A horrible wrapper around the unstable [Pattern API](https://github.com/rust-lang/rust/issues/56345).
pub enum Pattern<'r, F>
where
    F: FnMut(char) -> bool,
{
    Char(char),
    CharArrayRef(&'r [char]),
    Function(F),
    Str(&'r str),
    StrRef(&'r &'r str),
    StringRef(&'r String),
}

impl From<char> for Pattern<'_, fn(char) -> bool> {
    fn from(pat: char) -> Self {
        Pattern::Char(pat)
    }
}

impl<'r> From<&'r [char]> for Pattern<'r, fn(char) -> bool> {
    fn from(pat: &'r [char]) -> Self {
        Pattern::CharArrayRef(pat)
    }
}

impl<F> From<F> for Pattern<'_, F>
where
    F: FnMut(char) -> bool,
{
    fn from(pat: F) -> Self {
        Pattern::Function(pat)
    }
}

impl<'r> From<&'r str> for Pattern<'r, fn(char) -> bool> {
    fn from(pat: &'r str) -> Self {
        Pattern::Str(pat)
    }
}

impl<'r> From<&'r &'r str> for Pattern<'r, fn(char) -> bool> {
    fn from(pat: &'r &'r str) -> Self {
        Pattern::StrRef(pat)
    }
}

impl<'r> From<&'r String> for Pattern<'r, fn(char) -> bool> {
    fn from(pat: &'r String) -> Self {
        Pattern::StringRef(pat)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_starts_with() {
        assert_eq!(
            true,
            Slice::new("Hello World!").starts_with("Hello"),
            "`starts_with` is true",
        );
    }

    #[test]
    fn test_trim() {
        assert_eq!(
            "Hello World!",
            Slice::new("Hello World!").trim().slice(),
            "`trim` without any whitespace",
        );
        assert_eq!(
            "Hello World!",
            Slice::new("   Hello World!   ").trim().slice(),
            "`trim` with whitespace",
        );
    }

    #[test]
    fn test_trim_start() {
        assert_eq!(
            "Hello World!",
            Slice::new("Hello World!").trim_start().slice(),
            "`trim_start` without any whitespace",
        );
        assert_eq!(
            "Hello World!",
            Slice::new("   Hello World!").trim_start().slice(),
            "`trim_start` with whitespace",
        );
    }

    #[test]
    fn test_trim_end() {
        assert_eq!(
            "Hello World!",
            Slice::new("Hello World!").trim_end().slice(),
            "`trim_end` without any whitespace",
        );
        assert_eq!(
            "Hello World!",
            Slice::new("Hello World!   ").trim_end().slice(),
            "`trim_end` with whitespace",
        );
    }

    #[test]
    fn test_index() {
        let slice = Slice::new("Hello World!")
            .index(..11) // "Hello World"
            .index(6..) // "World"
            .index(..) // "World"
            .index(1..=2) // "or"
            .index(..);

        assert_eq!("or", slice.slice());
    }

    #[test]
    fn test_index_range_full() {
        let slice = Slice::new("Hello World!")
            .index(..)
            .index(..)
            .index(..)
            .index(..)
            .index(..);

        assert_eq!("Hello World!", slice.slice());
    }

    #[test]
    fn test_index_range() {
        let slice = Slice::new("Hello World!")
            .index(0..11) // "Hello World"
            .index(1..11) // "ello World"
            .index(2..8); // "lo Wor"

        assert_eq!("lo Wor", slice.slice());
    }

    #[test]
    fn test_index_range_inclusive() {
        let slice = Slice::new("Hello World!")
            .index(0..=11) // "Hello World!"
            .index(1..=8) // "ello Wor
            .index(0..=4); // "ello "

        assert_eq!("ello ", slice.slice());
    }
}
