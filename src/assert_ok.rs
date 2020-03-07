/// Asserts that expression returns `Ok(T)` variant.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let res: Result<i32, ()> = Ok(1);
///
/// assert_ok!(res);
/// # }
/// ```
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let res = Err(());
///
/// assert_ok!(res);  // Will panic
/// # }
/// ```
#[macro_export]
macro_rules! assert_ok {
    ($cond:expr) => {
        $crate::assert_ok!($cond,);
    };
    ($cond:expr,) => {
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
                panic!("assertion failed, expected Ok(..), got {:?}: {}", e, ::core::format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns `Ok(T)` variant in runtime.
#[macro_export]
macro_rules! debug_assert_ok {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_ok!($($arg)*); })
}
