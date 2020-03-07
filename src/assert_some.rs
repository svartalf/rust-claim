/// Asserts that expression returns `Some(T)` variant.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let maybe = Some(42);
///
/// assert_some!(maybe);
/// # }
/// ```
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let maybe = None;
///
/// assert_some!(maybe);  // Will panic
/// # }
/// ```
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
                panic!("assertion failed, expected Some(..), got None: {}", ::core::format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns `Some(T)` variant in runtime.
#[macro_export]
macro_rules! debug_assert_some {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_some!($($arg)*); })
}
