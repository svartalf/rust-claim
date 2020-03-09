/// Asserts that expression returns [`Ok(T)`] variant
/// and its value of `T` type equals to the right expression.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_ok_eq!`] for assertions that are not enabled in release builds by default.
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
/// assert_ok_eq!(res, 1);
///
/// // With custom messages
/// assert_ok_eq!(res, 1, "Everything is good with {:?}", res);
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
/// let value = assert_ok_eq!(res, 1);
/// assert_eq!(value, 1);
/// # }
/// ```
///
/// `Err(..)` variant will cause panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let res: Result<i32, ()> = Err(());
///
/// assert_ok_eq!(res, 1);  // Will panic
/// # }
/// ```
///
/// [`Ok(T)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Ok
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_ok_eq!`]: ./macro.debug_assert_ok_eq.html
#[macro_export]
macro_rules! assert_ok_eq {
    ($cond:expr, $expected:expr,) => {
        $crate::assert_ok_eq!($cond, $expected);
    };
    ($cond:expr, $expected:expr) => {
        match $cond {
            Ok(t) => {
                assert_eq!(t, $expected);
                t
            },
            e @ Err(..) => {
                panic!("assertion failed, expected Ok(..), got {:?}", e);
            }
        }
    };
    ($cond:expr, $expected:expr, $($arg:tt)+) => {
        match $cond {
            Ok(t) => {
                assert_eq!(t, $expected);
                t
            },
            e @ Err(..) => {
                panic!("assertion failed, expected Ok(..), got {:?}: {}", e, format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns [`Ok(T)`] variant in runtime.
///
/// Like [`assert_ok_eq!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`Ok(T)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Ok
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_ok_eq!`]: ./macro.assert_ok_eq.html
#[macro_export]
macro_rules! debug_assert_ok_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_ok_eq!($($arg)*); })
}
