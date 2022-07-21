/// A macro that allows you to clone, reference or copy variables captured by a
/// closure.
///
/// # Clone (default)
///
/// Without any other modifiers the default action taken for arguments is to
/// clone them.
///
/// ## Syntax
///
/// ```ignore
/// capture!(var)                          // let var = var.clone();
/// capture!(mut var)                      // let mut var = var.clone();
/// capture!(var1 => var2)                 // let var2 = var1.clone();
/// capture!(var1 => mut var2)             // let mut var2 = var1.clone();
/// ```
///
/// # Copy
///
/// Copying is possible my adding a `*` to the appropriate argument.
///
/// ## Syntax
///
/// ```ignore
/// capture!(*var)                         // let var = *var;
/// capture!(mut *var)                     // let mut var = *var;
/// capture!(*var1 => var2)                // let var2 = *var1;
/// capture!(*var1 => mut var2)            // let mut var2 = *var1;
/// ```
///
/// # References
///
/// You can reference a argument by adding the `ref` keyword to it.
///
/// ## Syntax
///
/// ```ignore
/// capture!(ref var)                      // let var = &var;
/// capture!(mut ref var)                  // let mut var = &var;
/// capture!(ref mut var)                  // let var = &mut var;
/// capture!(mut ref mut var)              // let mut var = &mut var;
/// capture!(ref var1 => var2)             // let var2 = &var1;
/// capture!(ref var1 => mut var2)         // let mut var2 = &var1;
/// capture!(ref mut var1 => var2)         // let var2 = &mut var1;
/// capture!(ref mut var1 => mut var2)     // let mut var2 = &mut var1;
/// ```
///
/// ## Note
///
/// References' syntax complexity increases along with you use case.
/// The order between `ref mut` and `mut ref` matters!
///
/// # Examples
///
/// ```rust
/// use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
///
/// fn test() {
///     let arc_atomic = Arc::new(AtomicBool::new(false));
///     let arc_mutex = Arc::new(Mutex::new(Ordering::AcqRel));
///
///     run({
///         fenn::capture!(arc_atomic, arc_mutex => mutex);
///
///         move || {
///             let lock = mutex.lock().unwrap();
///
///             arc_atomic.store(true, *lock);
///         }
///     });
/// }
///
/// fn run(f: impl FnOnce() -> ()) {
///     f();
/// }
/// ```
// TODO: figure out a way to pass closures as an argument so there doesn't need to be a block
#[macro_export]
macro_rules! capture {
    //#region [ rgba(27, 133, 184, 0.1) ] copy arguments
    // rename (with tokens)
    // c!(*var1 => mut var2)            let mut var2 = *var1;
    (@data, *$from:expr => mut $to:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let mut $to $(: $ty )? = *$from;

        $crate::capture!(@data, $( $tt )* )
    };
    // c!(*var1 => var2)                let var2 = *var1;
    (@data, *$from:expr => $to:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let $to $(: $ty )? = *$from;

        $crate::capture!(@data, $( $tt )* )
    };

    // rename (without tokens)
    // c!(*var1 => mut var2)            let mut var2 = *var1;
    (@data, *$from:expr => mut $to:ident $(: $ty:ty)? ) => {
        let mut $to $(: $ty )? = *$from;
    };
    // c!(*var1 => var2)                let var2 = *var1;
    (@data, *$from:expr => $to:ident $(: $ty:ty)? ) => {
        let $to $(: $ty )? = *$from;
    };

    // shadow (with tokens)
    // c!(mut *var)                     let mut var = *var;
    (@data, mut *$v:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let mut $v $(: $ty )? = *$v;

        $crate::capture!(@data, $( $tt )* )
    };
    // c!(*var)                         let var = *var;
    (@data, *$v:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let $v $(: $ty )? = *$v;

        $crate::capture!(@data, $( $tt )* )
    };

    // shadow (without tokens)
    // c!(mut *var)                     let mut var = *var;
    (@data, mut *$v:ident $(: $ty:ty)? ) => {
        let mut $v $(: $ty )? = *$v;
    };
    // c!(*var)                         let var = *var;
    (@data, *$v:ident $(: $ty:ty)? ) => {
        let $v $(: $ty )? = *$v;
    };
    //#endregion copy arguments


    //#region [ rgba(174, 90, 65, 0.1) ] reference arguments
    // rename (with tokens)
    // c!(ref mut var1 => mut var2)     let mut var2 = &mut var1;
    (@data, ref mut $from:expr => mut $to:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let mut $to $(: $ty )? = &mut $from;

        $crate::capture!(@data, $( $tt )* )
    };
    // c!(ref mut var1 => var2)         let var2 = &mut var1;
    (@data, ref mut $from:expr => $to:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let $to $(: $ty )? = &mut $from;

        $crate::capture!(@data, $( $tt )* )
    };
    // c!(ref var1 => mut var2)         let mut var2 = &var1;
    (@data, ref $from:expr => mut $to:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let mut $to $(: $ty )? = &$from;

        $crate::capture!(@data, $( $tt )* )
    };
    // c!(ref var1 => var2)             let var2 = &var1;
    (@data, ref $from:expr => $to:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let $to $(: $ty )? = &$from;

        $crate::capture!(@data, $( $tt )* )
    };

    // rename (without tokens)
    // c!(ref mut var1 => mut var2)     let mut var2 = &mut var1;
    (@data, ref mut $from:expr => mut $to:ident $(: $ty:ty)? ) => {
        let mut $to $(: $ty )? = &mut $from;
    };
    // c!(ref mut var1 => var2)         let var2 = &mut var1;
    (@data, ref mut $from:expr => $to:ident $(: $ty:ty)? ) => {
        let $to $(: $ty )? = &mut $from;
    };
    // c!(ref var1 => mut var2)         let mut var2 = &var1;
    (@data, ref $from:expr => mut $to:ident $(: $ty:ty)? ) => {
        let mut $to $(: $ty )? = &$from;
    };
    // c!(ref var1 => var2)             let var2 = &var1;
    (@data, ref $from:expr => $to:ident $(: $ty:ty)? ) => {
        let $to $(: $ty )? = &$from;
    };

    // shadow (with tokens)
    // c!(mut ref mut var)              let mut var = &mut var;
    (@data, mut ref mut $v:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let mut $v $(: $ty )? = &mut $v;

        $crate::capture!(@data, $( $tt )* )
    };
    // c!(mut ref var)                  let mut var = &var;
    (@data, mut ref $v:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let mut $v $(: $ty )? = &$v;

        $crate::capture!(@data, $( $tt )* )
    };
    // c!(ref mut var)                  let var = &mut var;
    (@data, ref mut $v:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let mut $v $(: $ty )? = &$v;

        $crate::capture!(@data, $( $tt )* )
    };
    // c!(ref var)                      let var = &var;
    (@data, ref $v:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let $v $(: $ty )? = &$v;

        $crate::capture!(@data, $( $tt )* )
    };

    // shadow (without tokens)
    // c!(mut ref mut var)              let mut var = &mut var;
    (@data, mut ref mut $v:ident $(: $ty:ty)? ) => {
        let mut $v $(: $ty )? = &mut $v;
    };
    // c!(mut ref var)                  let mut var = &var;
    (@data, mut ref $v:ident $(: $ty:ty)? ) => {
        let mut $v $(: $ty )? = &$v;
    };
    // c!(ref mut var)                  let var = &mut var;
    (@data, ref mut $v:ident $(: $ty:ty)? ) => {
        let $v $(: $ty )? = &mut $v;
    };
    // c!(ref var)                      let var = &var;
    (@data, ref $v:ident $(: $ty:ty)? ) => {
        let $v $(: $ty )? = &$v;
    };
    //#endregion reference arguments


    //#region [ rgba(85, 158, 131, 0.1) ] clone arguments
    // rename (with tokens)
    // c!(var1 => mut var2)             let mut var2 = var1.clone();
    (@data, $from:expr => mut $to:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let mut $to $(: $ty )? = $from.clone();

        $crate::capture!(@data, $( $tt )* )
    };
    // c!(var1 => var2)                 let var2 = var1.clone();
    (@data, $from:expr => $to:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let $to $(: $ty )? = $from.clone();

        $crate::capture!(@data, $( $tt )* )
    };

    // rename (without tokens)
    // c!(var1 => mut var2)             let mut var2 = var1.clone();
    (@data, $from:expr => mut $to:ident $(: $ty:ty)? ) => {
        let mut $to $(: $ty )? = $from.clone();
    };
    // c!(var1 => var2)                 let var2 = var1.clone();
    (@data, $from:expr => $to:ident $(: $ty:ty)? ) => {
        let $to $(: $ty )? = $from.clone();
    };

    // shadow (with tokens)
    // c!(mut var)                      let mut var = var.clone();
    (@data, mut $v:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let mut $v $(: $ty )? = $v.clone();

        $crate::capture!(@data, $( $tt )* )
    };
    // c!(var)                          let var = var.clone();
    (@data, $v:ident $(: $ty:ty)?, $( $tt:tt )* ) => {
        let $v $(: $ty )? = $v.clone();

        $crate::capture!(@data, $( $tt )* )
    };

    // shadow (without tokens)
    // c!(mut var)                      let mut var = var.clone();
    (@data, mut $v:ident $(: $ty:ty)? ) => {
        let mut $v $(: $ty )? = $v.clone();
    };
    // c!(var)                          let var = var.clone();
    (@data, $v:ident $(: $ty:ty)? ) => {
        let $v $(: $ty )? = $v.clone();
    };
    //#endregion clone arguments

    // No tokens
    (@data,) => {};


    ( $( $var_tt:tt )* ) => {
        $crate::capture! { @data, $( $var_tt )* }
    };
}
