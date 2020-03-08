/// Asserts that expression returns [`Poll::Ready(Ok(T))`] variant.
///
/// This macro is available for Rust 1.36+.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_ready_ok!`] for assertions that are not enabled in release builds by default.
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
/// assert_ready_ok!(res);
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
/// let value = assert_ready_ok!(res);
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
/// assert_ready_ok!(res);  // Will panic
/// # }
/// ```
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # use core::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, ()>> = Poll::Pending;
///
/// assert_ready_ok!(res);  // Will panic
/// # }
/// ```
///
/// [`Poll::Ready(Ok(T))`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
/// [`Poll::Pending`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Pending
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_ready_ok!`]: ./macro.debug_assert_ready_ok.html
#[macro_export]
macro_rules! assert_ready_ok {
    ($cond:expr,) => {
        $crate::assert_ready_ok!($cond);
    };
    ($cond:expr) => {
        match $cond {
            ::core::task::Poll::Ready(Ok(t)) => t,
            err_or_pending => {
                panic!("assertion failed, expected Ready(Ok(..)), got {:?}", err_or_pending);
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            ::core::task::Poll::Ready(Ok(t)) => t,
            err_or_pending => {
                panic!("assertion failed, expected Ready(Ok(..)), got {:?}: {}", err_or_pending, format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns [`Poll::Ready(Ok(T))`] variant in runtime.
///
/// Like [`assert_ready_ok!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`Poll::Ready(Ok(T))`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_ready_ok!`]: ./macro.assert_ready_ok.html
#[macro_export]
macro_rules! debug_assert_ready_ok {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_ready_ok!($($arg)*); })
}
