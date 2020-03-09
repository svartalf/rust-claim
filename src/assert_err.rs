/// Asserts that expression returns [`Err(E)`] variant.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_err!`] for assertions that are not enabled in release builds by default.
///
/// ## Custom messages
///
/// This macro has a second form, where a custom panic message can be provided
/// with or without arguments for formatting. See [`std::fmt`] for syntax for this form.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let res: Result<i32, ()> = Err(());
///
/// assert_err!(res);
///
/// // With custom messages
/// assert_err!(res, "we are checking if there was an error with {:?}", res);
/// # }
/// ```
///
/// `Ok(T)` variant will cause panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let res: Result<i32, ()> = Ok(42);
///
/// assert_err!(res);  // Will panic
/// # }
/// ```
///
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`Err(E)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Err
/// [`debug_assert_err!`]: ./macro.debug_assert_err.html
#[macro_export]
macro_rules! assert_err {
    ($cond:expr,) => {
        $crate::assert_err!($cond);
    };
    ($cond:expr) => {
        match $cond {
            t @ Ok(..) => {
                panic!("assertion failed, expected Err(..), got {:?}", t);
            },
            Err(e) => e,
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            t @ Ok(..) => {
                panic!("assertion failed, expected Err(..), got {:?}: {}", t, format_args!($($arg)+));
            },
            Err(e) => e,
        }
    };
}

/// Asserts that expression returns [`Err(E)`] variant in runtime.
///
/// Like [`assert_err!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`Err(E)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Err
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_err!`]: ./macro.assert_err.html
#[macro_export]
macro_rules! debug_assert_err {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_err!($($arg)*); })
}
