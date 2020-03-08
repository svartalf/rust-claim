/// Asserts that expression returns [`Some(T)`] variant.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_some!`] for assertions that are not enabled in release builds by default.
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
/// assert_some!(maybe);
///
/// // With custom messages
/// assert_some!(maybe, "Found it at {:?}", maybe);
/// # }
/// ```
///
/// `None` variant will cause panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let maybe = None;
///
/// assert_some!(maybe);  // Will panic
/// # }
/// ```
///
/// [`Some(T)`]: https://doc.rust-lang.org/core/option/enum.Option.html#variant.Some
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_none!`]: ./macro.debug_assert_none.html
#[macro_export]
macro_rules! assert_some {
    ($cond:expr) => {
        $crate::assert_some!($cond,);
    };
    ($cond:expr,) => {
        match $cond {
            ::core::option::Option::Some(t) => t,
            ::core::option::Option::None => {
                panic!("assertion failed, expected Some(..), got None");
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            ::core::option::Option::Some(t) => t,
            ::core::option::Option::None => {
                panic!("assertion failed, expected Some(..), got None: {}", format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns [`Some(T)`] variant in runtime.
///
/// Like [`assert_some!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`Some(T)`]: https://doc.rust-lang.org/core/option/enum.Option.html#variant.Some
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_some!`]: ./macro.assert_some.html
#[macro_export]
macro_rules! debug_assert_some {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_some!($($arg)*); })
}
