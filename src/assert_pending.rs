/// Asserts that expression returns [`Poll::Pending`] variant.
///
/// This macro is available for Rust 1.36+.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_pending!`] for assertions that are not enabled in release builds by default.
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
/// let res: Poll<i32> = Poll::Pending;
///
/// assert_pending!(res);
///
/// // With custom messages
/// assert_pending!(res, "Future is not ready yet");
/// # }
/// ```
///
/// [`Poll::Pending`] variant will also be returned from this macro call:
///
/// ```rust
/// # #[macro_use] extern crate claim;
/// # use core::task::Poll;
/// # fn main() {
/// let res: Poll<i32> = Poll::Pending;
///
/// let value = assert_pending!(res);
/// assert_eq!(value, Poll::Pending);
/// # }
/// ```
///
/// [`Poll::Ready(T)`] variant will cause panic:
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
/// [`Poll::Ready(T)`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
/// [`Poll::Pending`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Pending
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_pending!`]: ./macro.debug_assert_pending.html
#[macro_export]
macro_rules! assert_pending {
    ($cond:expr,) => {
        $crate::assert_pending!($cond);
    };
    ($cond:expr) => {
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
                panic!("assertion failed, expected Pending, got {:?}", format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns [`Poll::Pending`] variant in runtime.
///
/// This macro is available for Rust 1.36+.
///
/// Like [`assert_pending!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`Poll::Pending`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Pending
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_pending!`]: ./macro.assert_pending.html
#[macro_export]
macro_rules! debug_assert_pending {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_pending!($($arg)*); })
}
