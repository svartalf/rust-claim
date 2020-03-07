/// Asserts that expression returns `Err(E)` variant.
///
/// ## Examples
///
/// ```rust
/// let res: Result<i32, ()> = Err(());
///
/// claim::assert_err!(res);
/// ```
///
/// ```rust,should_panic
/// let res: Result<i32, ()> = Ok(42);
///
/// claim::assert_err!(res);  // Will panic
/// ```
#[macro_export]
macro_rules! assert_err {
    ($cond:expr) => {
        $crate::assert_err!($cond,);
    };
    ($cond:expr,) => {
        match $cond {
            t @ ::core::result::Result::Ok(..) => {
                panic!("assertion failed, expected Err(..), got {:?}", t);
            },
            ::core::result::Result::Err(e) => e,
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            t @ ::core::result::Result::Ok(..) => {
                panic!("assertion failed, expected Err(..), got {:?}: {}", t, ::core::format_args!($($arg)+));
            },
            ::core::result::Result::Err(e) => {
                e
            }
        }
    };
}

/// Asserts that expression returns `Err(E)` variant in runtime.
#[macro_export]
macro_rules! debug_assert_err {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_err!($($arg)*); })
}
