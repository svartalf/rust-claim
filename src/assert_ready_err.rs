/// Asserts that expression returns [`Poll::Ready(Err(E))`] variant.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use] extern crate claim;
/// # use core::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, ()>> = Poll::Ready(Err(()));
///
/// assert_ready_err!(res);
/// # }
/// ```
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # use core::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, ()>> = Poll::Ready(Ok(42));
///
/// assert_ready_err!(res);  // Will panic
/// # }
/// ```

/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # use core::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, ()>> = Poll::Pending;
///
/// assert_ready_err!(res);  // Will panic
/// # }
/// ```
///
/// [`Poll::Ready(T)`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
#[macro_export]
macro_rules! assert_ready_err {
    ($cond:expr) => {
        $crate::assert_ready_err!($cond,);
    };
    ($cond:expr,) => {
        match $cond {
            ::core::task::Poll::Ready(Err(e)) => e,
            ok_or_pending => {
                panic!("assertion failed, expected Ready(Err(..)), got {:?}", ok_or_pending);
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            ::core::task::Poll::Ready(Err(e)) => e,
            ok_or_pending => {
                panic!("assertion failed, expected Ready(Err(..)), got {:?}: {}", ok_or_pending, ::core::format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns `Poll::Ready(Err(T))` variant in runtime.
#[macro_export]
macro_rules! debug_assert_ready_err {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_ready_err!($($arg)*); })
}
