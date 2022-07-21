#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

#[macro_use]
mod capture;
mod newtype;
mod peep;
mod wrap;

pub mod iter;
pub mod slice;

pub use crate::{
    iter::{ChainIf, ChainIfElse, IntoIteratorExt},
    peep::{Peep, PeepOption, PeepResult},
    wrap::Wrap,
};

#[cfg(any(feature = "std", feature = "hashbrown"))]
#[cfg_attr(any(feature = "std", feature = "hashbrown"), macro_use)]
mod hash;
#[cfg(feature = "std")]
mod io;
#[cfg(feature = "alloc")]
mod string;
#[cfg(feature = "alloc")]
mod vec;
#[cfg(feature = "alloc")]
pub mod vec_map;

#[cfg(any(feature = "std", feature = "hashbrown"))]
pub use crate::hash::HashMapExt;
#[cfg(feature = "std")]
pub use crate::io::{Endian, ReadExt, WriteExt};
#[cfg(feature = "alloc")]
pub use crate::string::StringExt;
#[cfg(feature = "alloc")]
pub use crate::vec::VecExt;
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "alloc", doc(inline))]
pub use crate::vec_map::VecMap;

/// A macro used to create a new [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html).
///
/// By itself it isn't all that useful, but it makes use when creating tuples
/// of [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)s.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
///
/// assert_eq!(Arc::new(1), fenn::arc!(1));
/// ```
///
/// ```
/// use std::sync::Arc;
///
/// assert_eq!((Arc::new(1), Arc::new(2)), fenn::arc!(1, 2));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "alloc", macro_export)]
macro_rules! arc {
    ( $item:expr ) => {
        $crate::lib::Arc::new($item)
    };
    ( $item:expr, ) => {
        $crate::arc!($item)
    };
    ( $( $item:expr ),+ ) => {
        ( $( $crate::arc!($item) ),+)
    };
}

/// A macro used to create a new [`Box`](https://doc.rust-lang.org/std/boxed/struct.Arc.html).
///
/// By itself it isn't all that useful, but it makes use when creating tuples
/// of [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html)s.
///
/// # Examples
///
/// ```
/// use std::boxed::Box;
///
/// assert_eq!(Box::new(1), fenn::boxed!(1));
/// ```
///
/// ```
/// use std::boxed::Box;
///
/// assert_eq!((Box::new(1), Box::new(2)), fenn::boxed!(1, 2));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "alloc", macro_export)]
macro_rules! boxed {
    ( $item:expr ) => {
        $crate::lib::Box::new($item)
    };
    ( $item:expr, ) => {
        $crate::boxed!($item)
    };
    ( $( $item:expr ),+ ) => {
        ( $( $crate::boxed!($item) ),+)
    };
}

/// Various extensions to the bool primitive.
///
/// At the moment its only a stable version of [`then_some`](https://doc.rust-lang.org/std/primitive.bool.html#method.then_some) and
/// [`then`](https://doc.rust-lang.org/std/primitive.bool.html#method.then).
pub trait BoolExt: Copy {
    /// A stable version of [`then_some`](https://doc.rust-lang.org/std/primitive.bool.html#method.then_some).
    ///
    /// Returns `Some(f())` if the `bool` is `true`, or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::BoolExt;
    ///
    /// assert_eq!(false.some(0), None);
    /// assert_eq!(true.some(0), Some(0));
    /// ```
    fn some<T>(self, t: T) -> Option<T>;

    /// A stable version of [`then`](https://doc.rust-lang.org/std/primitive.bool.html#method.then).
    ///
    /// Returns `Some(f())` if the `bool` is `true`, or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::BoolExt;
    ///
    /// assert_eq!(false.else_some(|| 0), None);
    /// assert_eq!(true.else_some(|| 0), Some(0));
    /// ```
    fn else_some<T>(self, f: impl FnOnce() -> T) -> Option<T>;
}

impl BoolExt for bool {
    fn some<T>(self, t: T) -> Option<T> {
        if self {
            Some(t)
        } else {
            None
        }
    }

    fn else_some<T>(self, f: impl FnOnce() -> T) -> Option<T> {
        self.some(f())
    }
}

/// Wrap `Self` in a [`Option`] based off a predicate.
///
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
pub trait IntoOption
where
    Self: Sized,
{
    /// Results `Some(self)` if the predicate returns `true`, or `None` otherwise.
    fn some_if(self, predicate: bool) -> Option<Self> {
        if predicate {
            Some(self)
        } else {
            None
        }
    }

    /// Results `Some(self)` if the predicate returns `true`, or `None` otherwise.
    fn with_some_if<F>(self, predicate: F) -> Option<Self>
    where
        F: FnOnce(&Self) -> bool,
    {
        let res = predicate(&self);

        self.some_if(res)
    }

    /// Results `None` if the predicate returns `true`, or `Some(self)` otherwise.
    fn none_if(self, predicate: bool) -> Option<Self> {
        if predicate {
            None
        } else {
            Some(self)
        }
    }

    /// Results `None` if the predicate returns `true`, or `Some(self)` otherwise.
    fn with_none_if<F>(self, predicate: F) -> Option<Self>
    where
        F: FnOnce(&Self) -> bool,
    {
        let res = predicate(&self);

        self.none_if(res)
    }
}

impl<T> IntoOption for T {}

/// Wrap `Self` in a [`Result`] based off a predicate.
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
pub trait IntoResult
where
    Self: Sized,
{
    /// Results `Ok(self)` if the predicate returns `true`, or `Err(err)` otherwise.
    fn ok_if<E>(self, predicate: bool, err: E) -> Result<Self, E> {
        if predicate {
            Ok(self)
        } else {
            Err(err)
        }
    }

    /// Results `Ok(self)` if the predicate returns `true`, or `Err(err)` otherwise.
    fn with_ok_if<F, E>(self, predicate: F, err: E) -> Result<Self, E>
    where
        F: FnOnce(&Self) -> bool,
    {
        let res = predicate(&self);

        self.ok_if(res, err)
    }

    /// Results `Err(err)` if the predicate returns `true`, or `Ok(self)` otherwise.
    fn err_if<E>(self, predicate: bool, err: E) -> Result<Self, E> {
        if predicate {
            Err(err)
        } else {
            Ok(self)
        }
    }

    /// Results `Err(err)` if the predicate returns `true`, or `Ok(self)` otherwise.
    fn with_err_if<F, E>(self, predicate: F, err: E) -> Result<Self, E>
    where
        F: FnOnce(&Self) -> bool,
    {
        let res = predicate(&self);

        self.err_if(res, err)
    }
}

impl<T> IntoResult for T {}

/// Various extensions to
/// [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html).
pub trait OptionExt {
    /// The item type inside the `Option`.
    type Item;

    /// Take the value of `self` only if the given condition is `true`.
    fn take_if<F>(&mut self, fun: F) -> Option<Self::Item>
    where
        F: FnOnce(&Self::Item) -> bool;
}

impl<T> OptionExt for Option<T> {
    type Item = T;

    fn take_if<F>(&mut self, fun: F) -> Option<Self::Item>
    where
        F: FnOnce(&Self::Item) -> bool,
    {
        if self.as_ref().map(fun).unwrap_or_else(|| false) {
            self.take()
        } else {
            None
        }
    }
}

#[doc(hidden)]
pub mod lib {
    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::boxed::Box;
    #[cfg(feature = "std")]
    pub use std::boxed::Box;

    #[cfg(all(feature = "hashbrown", not(feature = "std")))]
    pub use hashbrown::HashMap;
    #[cfg(feature = "std")]
    pub use std::collections::HashMap;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::fmt;
    #[cfg(feature = "std")]
    pub use std::fmt;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::string::String;
    #[cfg(feature = "std")]
    pub use std::string::String;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::sync::Arc;
    #[cfg(feature = "std")]
    pub use std::sync::Arc;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::vec;
    #[cfg(feature = "std")]
    pub use std::vec;
}
