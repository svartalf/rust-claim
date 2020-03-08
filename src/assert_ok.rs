/// Asserts that expression returns [`Ok(T)`] variant.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_ok!`] for assertions that are not enabled in release builds by default.
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
/// let res: Result<i32, ()> = Ok(1);
///
/// assert_ok!(res);
///
/// // With custom messages
/// assert_ok!(res, "Everything is good with {:?}", res);
/// # }
/// ```
///
/// Value of `T` type from `Ok(T)` will be returned from the macro call:
///
/// ```rust
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let res: Result<i32, ()> = Ok(1);
///
/// let value = assert_ok!(res);
/// assert_eq!(value, 1);
/// # }
/// ```
///
/// `Err(..)` variant will cause panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let res = Err(());
///
/// assert_ok!(res);  // Will panic
/// # }
/// ```
///
/// [`Ok(T)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Ok
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_ok!`]: ./macro.debug_assert_ok.html
#[macro_export]
macro_rules! assert_ok {
    ($cond:expr,) => {
        $crate::assert_ok!($cond);
    };
    ($cond:expr) => {
        match $cond {
            ::core::result::Result::Ok(t) => t,
            e @ ::core::result::Result::Err(..) => {
                panic!("assertion failed, expected Ok(..), got {:?}", e);
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            ::core::result::Result::Ok(t) => t,
            e @ ::core::result::Result::Err(..) => {
                panic!("assertion failed, expected Ok(..), got {:?}: {}", e, format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns [`Ok(T)`] variant in runtime.
///
/// Like [`assert_ok!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`Ok(T)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Ok
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_ok!`]: ./macro.assert_ok.html
#[macro_export]
macro_rules! debug_assert_ok {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_ok!($($arg)*); })
}
