/// Asserts that left expression returns [`Some(T)`] variant
/// and its value of `T` type equals to the right expression.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_some_eq!`] for assertions that are not enabled in release builds by default.
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
/// let maybe = Some(42);
///
/// assert_some_eq!(maybe, 42);
///
/// // With custom messages
/// assert_some_eq!(maybe, 42, "Got some value");
/// # }
/// ```
///
/// `None` variant will cause panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let maybe: Option<i32> = None;
///
/// assert_some_eq!(maybe, 42);  // Will panic
/// # }
/// ```
///
/// [`Some(T)`]: https://doc.rust-lang.org/core/option/enum.Option.html#variant.Some
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_some_eq!`]: ./macro.debug_assert_some_eq.html
#[macro_export]
macro_rules! assert_some_eq {
    ($cond:expr, $expected:expr,) => {
        $crate::assert_some_eq!($cond, $expected);
    };
    ($cond:expr, $expected:expr) => {
        match $cond {
            Some(t) => {
                assert_eq!(t, $expected);
                t
            },
            None => {
                panic!("assertion failed, expected Some(..), got None");
            }
        }
    };
    ($cond:expr, $expected:expr, $($arg:tt)+) => {
        match $cond {
            Some(t) => {
                assert_eq!(t, $expected);
                t
            },
            None => {
                panic!("assertion failed, expected Some(..), got None: {}", format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns [`Some(T)`] variant in runtime.
///
/// Like [`assert_some_eq!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`Some(T)`]: https://doc.rust-lang.org/core/option/enum.Option.html#variant.Some
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_some_eq!`]: ./macro.assert_some_eq.html
#[macro_export]
macro_rules! debug_assert_some_eq {
    ($($arg:tt)*) => (if core::cfg!(debug_assertions) { $crate::assert_some_eq!($($arg)*); })
}
