/// Asserts that expression returns [`None`] variant.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let maybe: Option<i32> = None;
///
/// assert_none!(maybe);
/// # }
/// ```
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
#[macro_export]
macro_rules! assert_none {
    ($cond:expr) => {
        $crate::assert_none!($cond,);
    };
    ($cond:expr,) => {
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
                panic!("assertion failed, expected None, got {:?}: {}", t, ::core::format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns [`None`] variant in runtime.
///
/// [`None`]: https://doc.rust-lang.org/core/option/enum.Option.html#variant.None
#[macro_export]
macro_rules! debug_assert_none {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_none!($($arg)*); })
}
