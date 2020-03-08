/// Asserts that expression returns [`None`] variant.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_none!`] for assertions that are not enabled in release builds by default.
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
/// let maybe: Option<i32> = None;
///
/// assert_none!(maybe);
///
/// // With custom messages
/// assert_none!(maybe, "Yep, there is nothing in here");
/// assert_none!(maybe, "we asserting that there are no droids we are looking for at {:?}", maybe);
/// # }
/// ```
///
/// `Some(T)` variant will cause panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let maybe = Some(42i32);
///
/// assert_none!(maybe);  // Will panic
/// # }
/// ```
///
/// [`None`]: https://doc.rust-lang.org/core/option/enum.Option.html#variant.None
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_none!`]: ./macro.debug_assert_none.html
#[macro_export]
macro_rules! assert_none {
    ($cond:expr,) => {
        $crate::assert_none!($cond);
    };
    ($cond:expr) => {
        match $cond {
            n @ ::core::option::Option::None => n,
            t @ ::core::option::Option::Some(..) => {
                panic!("assertion failed, expected None, got {:?}", t);
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            n @ ::core::option::Option::None => n,
            t @ ::core::option::Option::Some(..) => {
                panic!("assertion failed, expected None, got {:?}: {}", t, format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns [`None`] variant in runtime.
///
/// Like [`assert_none!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`None`]: https://doc.rust-lang.org/core/option/enum.Option.html#variant.None
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_none!`]: ./macro.assert_none.html
#[macro_export]
macro_rules! debug_assert_none {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_none!($($arg)*); })
}
