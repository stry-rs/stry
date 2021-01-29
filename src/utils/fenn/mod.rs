//! This module contains part of my general util library `fenn`, and is used as a 'testing' environment.

#![deny(unsafe_code)]

mod capture;
mod hash;
pub mod iter;
mod newtype;
mod peep;
mod string;
mod vec;
pub mod vec_map;
mod wrap;

pub use self::{
    hash::HashMapExt,
    peep::{PeekOption, Peep, PeepResult},
    string::StringExt,
    vec::VecExt,
    wrap::Wrap,
};

/// A [`try!`] like macro for `Result<Option<_>, _>`s.
///
/// [`try!`]: https://doc.rust-lang.org/std/macro.try.html
#[macro_export]
macro_rules! try_res_opt {
    ($inner:expr) => {
        match $inner {
            ::core::result::Result::Ok(::core::option::Option::Some(value)) => {
                ::core::result::Result::Ok(value)
            }
            ::core::result::Result::Ok(::core::option::Option::None) => {
                return ::core::result::Result::Ok(::core::option::Option::None)
            }
            ::core::result::Result::Err(err) => return ::core::result::Result::Err(err),
        }
    };
    ($inner:expr,) => {
        $crate::try_res_opt!($inner)
    };
}

/// Various extensions to the bool primitive.
pub trait BoolExt: Copy {
    /// A stable version of [`then_some`](https://doc.rust-lang.org/std/primitive.bool.html#method.then_some).
    ///
    /// Returns `Some(t)` if the `bool` is `true`, or `None` otherwise.
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
    /// assert_eq!(false.some_with(|| 0), None);
    /// assert_eq!(true.some_with(|| 0), Some(0));
    /// ```
    fn some_with<T, F>(self, run: F) -> Option<T>
    where
        F: FnOnce() -> T;

    /// Returns `Some(())` if the `bool` is `true`, or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::BoolExt;
    ///
    /// assert!(false.as_option().is_none());
    /// assert!(true.as_option().is_some());
    /// ```
    fn as_option(self) -> Option<()>;

    /// Returns `Ok(t)` if the `bool` is `true`, or `Er(())` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::BoolExt;
    ///
    /// assert_eq!(false.ok(0), Err(()));
    /// assert_eq!(true.ok(0), Ok(0));
    /// ```
    fn ok<T>(self, t: T) -> Result<T, ()>;

    /// Returns `Ok(run())` if the `bool` is `true`, or `Er(())` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fenn::BoolExt;
    ///
    /// assert_eq!(false.ok_with(|| 0), Err(()));
    /// assert_eq!(true.ok_with(|| 0), Ok(0));
    /// ```

    fn ok_with<T, F>(self, run: F) -> Result<T, ()>
    where
        F: FnOnce() -> T;

    /// Returns `Ok(())` if the `bool` is `true`, or `Err(())` otherwise.
    fn as_result(self) -> Result<(), ()>;
}

impl BoolExt for bool {
    fn some<T>(self, t: T) -> Option<T> {
        if self {
            Some(t)
        } else {
            None
        }
    }

    fn some_with<T, F>(self, run: F) -> Option<T>
    where
        F: FnOnce() -> T,
    {
        self.some(run())
    }

    fn as_option(self) -> Option<()> {
        self.some(())
    }

    fn ok<T>(self, t: T) -> Result<T, ()> {
        if self {
            Ok(t)
        } else {
            Err(())
        }
    }

    fn ok_with<T, F>(self, run: F) -> Result<T, ()>
    where
        F: FnOnce() -> T,
    {
        self.ok(run())
    }

    fn as_result(self) -> Result<(), ()> {
        self.ok(())
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
        if self.as_ref().map(fun).unwrap_or(false) {
            self.take()
        } else {
            None
        }
    }
}

#[doc(hidden)]
pub mod lib {
    pub use std::boxed::Box;
    pub use std::collections::HashMap;
    pub use std::fmt;
    pub use std::string::String;
    pub use std::sync::Arc;
    pub use std::vec;
}
