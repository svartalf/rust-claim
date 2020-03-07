/// Asserts that left expression returns [`Poll::Ready(Ok(T))`] variant
/// and it value of `T` type equals to the right expression.
///
/// This macro is available for Rust 1.36+.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use] extern crate claim;
/// # use core::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, ()>> = Poll::Ready(Ok(42));
///
/// assert_ready_eq!(res, 42);
/// # }
/// ```
///
/// Value of `T` type from the `Poll::Ready(Ok(T))` will also be returned from this macro call:
///
/// ```rust
/// # #[macro_use] extern crate claim;
/// # use core::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, ()>> = Poll::Ready(Ok(42));
///
/// let value = assert_ready_eq!(res, 42);
/// assert_eq!(value, 42);
/// # }
/// ```
///
/// Both `Poll::Ready(Err(..))` and `Poll::Pending` variants will cause panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # use core::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, ()>> = Poll::Ready(Err(()));
///
/// assert_ready_eq!(res, 42);  // Will panic
/// # }
/// ```
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # use core::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, ()>> = Poll::Pending;
///
/// assert_ready_eq!(res, 42);  // Will panic
/// # }
/// ```
///
/// [`Poll::Ready(Ok(T))`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
#[macro_export]
macro_rules! assert_ready_eq {
    ($cond:expr, $expected:expr,) => {
        $crate::assert_ready_eq!($cond, $expected);
    };
    ($cond:expr, $expected:expr) => {
        match $cond {
            ::core::task::Poll::Ready(Ok(t)) => {
                assert_eq!(t, $expected);
                t
            },
            err_or_pending => {
                panic!("assertion failed, expected Ready(Ok(..)), got {:?}", err_or_pending);
            }
        }
    };
    ($cond:expr, $expected:expr, $($arg:tt)+) => {
        match $cond {
            ::core::task::Poll::Ready(Ok(t)) => {
                assert_eq!(t, $expected, $($arg)+);
                t
            },
            err_or_pending => {
                panic!("assertion failed, expected Ready(Ok(..)), got {:?}: {}", err_or_pending, ::core::format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that left expression returns [`Poll::Ready(Ok(T))`] variant
/// and it value of `T` type equals to the right expression in runtime.
///
/// [`Poll::Ready(Ok(T))`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
#[macro_export]
macro_rules! debug_assert_ready_eq {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_ready_eq!($($arg)*); })
}
