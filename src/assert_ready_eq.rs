/// Asserts that left expression returns [`Poll::Ready(Ok(T))`] variant
/// and its value of `T` type equals to the right expression.
///
/// This macro is available for Rust 1.36+.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_ready_eq!`] for assertions that are not enabled in release builds by default.
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
/// Both `Poll::Ready(Err(..))` and [`Poll::Pending`] variants will cause panic:
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
/// [`Poll::Pending`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Pending
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_ready!`]: ./macro.debug_assert_ready.html
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
                panic!("assertion failed, expected Ready(Ok(..)), got {:?}: {}", err_or_pending, format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that left expression returns [`Poll::Ready(Ok(T))`] variant
/// and its value of `T` type equals to the right expression in runtime.
///
/// Like [`assert_ready_eq!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`Poll::Ready(Ok(T))`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_ready_eq!`]: ./macro.assert_ready_eq.html
#[macro_export]
macro_rules! debug_assert_ready_eq {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_ready_eq!($($arg)*); })
}
