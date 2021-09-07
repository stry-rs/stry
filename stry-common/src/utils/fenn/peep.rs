// Based off https://www.reddit.com/r/rust/comments/6nmw07/method_naming_convention/dkbr285/

/// This trait allows for 'peeping' at values in chained function calls.
///
/// ```rust
/// use fenn::Peep;
///
/// let vec = vec![1, 2, 3]
///     .into_iter()
///     .map(|i| i * 2)
///     .peep(|vec| {
///         // This is only for the assert and isn't normally needed
///         let temp_vec: Vec<_> = vec.clone().collect();
///
///          assert_eq!(temp_vec, [2, 4, 6]);
///      })
///     .map(|i| i / 2)
///     .peep(|vec| {
///         // Again this is only for the assert
///         let temp_vec: Vec<_> = vec.clone().collect();
///
///          assert_eq!(temp_vec, [1, 2, 3]);
///      })
///     .collect::<Vec<usize>>();
/// ```
pub trait Peep: Sized {
    fn peep<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&Self) -> R,
        R: Sized,
    {
        run(&self);

        self
    }

    #[allow(unused_variables)]
    fn peep_dbg<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&Self) -> R,
        R: Sized,
    {
        #[cfg(debug_assertions)]
        return self.peep(run);

        #[cfg(not(debug_assertions))]
        return self;
    }

    fn peep_mut<F, R>(mut self, run: F) -> Self
    where
        F: FnOnce(&mut Self) -> R,
        R: Sized,
    {
        run(&mut self);

        self
    }

    #[allow(unused_variables)]
    fn peep_mut_dbg<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&mut Self) -> R,
        R: Sized,
    {
        #[cfg(debug_assertions)]
        return self.peep_mut(run);

        #[cfg(not(debug_assertions))]
        return self;
    }
}

impl<T: Sized> Peep for T {}

/// [`Peep`](./trait.Peep.html), but specifically for
/// [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)'s.
pub trait PeepOption<T: Sized>: Sized {
    fn peep_some<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&T) -> R,
        R: Sized;

    #[allow(unused_variables)]
    fn peep_some_dbg<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&T) -> R,
        R: Sized,
    {
        #[cfg(debug_assertions)]
        return self.peep_some(run);

        #[cfg(not(debug_assertions))]
        return self;
    }

    fn peep_some_mut<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&mut T) -> R,
        R: Sized;

    #[allow(unused_variables)]
    fn peep_some_mut_dbg<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&mut T) -> R,
        R: Sized,
    {
        #[cfg(debug_assertions)]
        return self.peep_some_mut(run);

        #[cfg(not(debug_assertions))]
        return self;
    }

    fn peep_none<F, R>(self, run: F) -> Self
    where
        F: FnOnce() -> R,
        R: Sized;

    #[allow(unused_variables)]
    fn peep_none_dbg<F, R>(self, run: F) -> Self
    where
        F: FnOnce() -> R,
        R: Sized,
    {
        #[cfg(debug_assertions)]
        return self.peep_none(run);

        #[cfg(not(debug_assertions))]
        return self;
    }
}

impl<T: Sized> PeepOption<T> for Option<T> {
    fn peep_some<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&T) -> R,
        R: Sized,
    {
        if let Some(inner) = self.as_ref() {
            run(inner);
        }

        self
    }

    fn peep_some_mut<F, R>(mut self, run: F) -> Self
    where
        F: FnOnce(&mut T) -> R,
        R: Sized,
    {
        if let Some(inner) = self.as_mut() {
            run(inner);
        }

        self
    }

    fn peep_none<F, R>(self, run: F) -> Self
    where
        F: FnOnce() -> R,
        R: Sized,
    {
        if self.is_none() {
            run();
        }

        self
    }
}

/// [`Peep`](./trait.Peep.html), but specifically for
/// [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html)'s.
pub trait PeepResult<T: Sized, E: Sized>: Sized {
    fn peek_ok<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&T) -> R,
        R: Sized;

    #[allow(unused_variables)]
    fn peek_ok_dbg<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&T) -> R,
        R: Sized,
    {
        #[cfg(debug_assertions)]
        return self.peek_ok(run);

        #[cfg(not(debug_assertions))]
        return self;
    }

    fn peek_ok_mut<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&mut T) -> R,
        R: Sized;

    #[allow(unused_variables)]
    fn peek_ok_mut_dbg<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&mut T) -> R,
        R: Sized,
    {
        #[cfg(debug_assertions)]
        return self.peek_ok_mut(run);

        #[cfg(not(debug_assertions))]
        return self;
    }

    fn peek_err<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&E) -> R,
        R: Sized;

    #[allow(unused_variables)]
    fn peek_err_dbg<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&E) -> R,
        R: Sized,
    {
        #[cfg(debug_assertions)]
        return self.peek_err(run);

        #[cfg(not(debug_assertions))]
        return self;
    }

    fn peek_err_mut<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&mut E) -> R,
        R: Sized;

    #[allow(unused_variables)]
    fn peek_err_mut_dbg<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&mut E) -> R,
        R: Sized,
    {
        #[cfg(debug_assertions)]
        return self.peek_err_mut(run);

        #[cfg(not(debug_assertions))]
        return self;
    }
}

impl<T: Sized, E: Sized> PeepResult<T, E> for Result<T, E> {
    fn peek_ok<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&T) -> R,
        R: Sized,
    {
        if let Ok(inner) = self.as_ref() {
            run(inner);
        }

        self
    }

    fn peek_ok_mut<F, R>(mut self, run: F) -> Self
    where
        F: FnOnce(&mut T) -> R,
        R: Sized,
    {
        if let Ok(inner) = self.as_mut() {
            run(inner);
        }

        self
    }

    fn peek_err<F, R>(self, run: F) -> Self
    where
        F: FnOnce(&E) -> R,
        R: Sized,
    {
        if let Err(inner) = self.as_ref() {
            run(inner);
        }

        self
    }

    fn peek_err_mut<F, R>(mut self, run: F) -> Self
    where
        F: FnOnce(&mut E) -> R,
        R: Sized,
    {
        if let Err(inner) = self.as_mut() {
            run(inner);
        }

        self
    }
}
