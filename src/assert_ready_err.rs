/// Asserts that expression returns [`Poll::Ready(Err(E))`] variant.
///
/// This macro is available for Rust 1.36+.
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
/// Value of `E` type from the `Poll::Ready(Err(E))` will also be returned from this macro call:
///
/// ```rust
/// # #[macro_use] extern crate claim;
/// # use core::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, String>> = Poll::Ready(Err("Something went wrong".to_string()));
///
/// let message = assert_ready_err!(res);
/// assert_eq!("Something went wrong", message);
/// # }
/// ```
///
/// Both `Poll::Ready(Ok(..))` and `Poll::Pending` variants will cause panic:
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
/// [`Poll::Ready(Err(E))`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
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

/// Asserts that expression returns [`Poll::Ready(Err(E))`] variant in runtime.
///
/// [`Poll::Ready(Err(E))`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
#[macro_export]
macro_rules! debug_assert_ready_err {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_ready_err!($($arg)*); })
}
