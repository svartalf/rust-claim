/// Asserts that expression returns [`Poll::Pending`] variant.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use] extern crate claim;
/// # use core::task::Poll;
/// # fn main() {
/// let res: Poll<i32> = Poll::Pending;
///
/// assert_pending!(res);
/// # }
/// ```
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # use core::task::Poll;
/// # fn main() {
/// let res = Poll::Ready(42);
///
/// assert_pending!(res);  // Will panic
/// # }
/// ```
///
/// [`Poll::Pending`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Pending
#[macro_export]
macro_rules! assert_pending {
    ($cond:expr) => {
        $crate::assert_pending!($cond,);
    };
    ($cond:expr,) => {
        match $cond {
            p @ ::core::task::Poll::Pending => p,
            r @ ::core::task::Poll::Ready(..) => {
                panic!("assertion failed, expected Pending, got {:?}", r);
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            p @ ::core::task::Poll::Pending => p,
            r @ ::core::task::Poll::Ready(..) => {
                panic!("assertion failed, expected Pending, got {:?}", ::core::format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns `Poll::Pending` variant in runtime.
#[macro_export]
macro_rules! debug_assert_pending {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_pending!($($arg)*); })
}
