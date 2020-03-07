/// Asserts that expression returns [`Poll::Ready(T)`] variant.
///
/// This macro is available for Rust 1.36+.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use] extern crate claim;
/// # use core::task::Poll;
/// # fn main() {
/// let res = Poll::Ready(42);
///
/// assert_ready!(res);
/// # }
/// ```
///
/// Value of `T` type from the `Poll::Ready(T)` will also be returned from this macro call:
///
/// ```rust
/// # #[macro_use] extern crate claim;
/// # use core::task::Poll;
/// # fn main() {
/// let res = Poll::Ready(42);
///
/// let value = assert_ready!(res);
/// assert_eq!(value, 42);
/// # }
/// ```
///
/// [`Poll::Pending`] variant will cause panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # use core::task::Poll;
/// # fn main() {
/// let res = Poll::Pending;
///
/// assert_ready!(res);  // Will panic
/// # }
/// ```
///
/// [`Poll::Ready(T)`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
/// [`Poll::Pending`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Pending
#[macro_export]
macro_rules! assert_ready {
    ($cond:expr) => {
        $crate::assert_ready!($cond,);
    };
    ($cond:expr,) => {
        match $cond {
            ::core::task::Poll::Ready(t) => t,
            p @ ::core::task::Poll::Pending => {
                panic!("assertion failed, expected Ready(..), got {:?}", p);
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            ::core::task::Poll::Ready(t) => t,
            p @ ::core::task::Poll::Pending => {
                panic!("assertion failed, expected Ready(..), got {:?}", ::core::format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns [`Poll::Ready(T)`] variant in runtime.
///
/// [`Poll::Ready(T)`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
#[macro_export]
macro_rules! debug_assert_ready {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_ready!($($arg)*); })
}
