/// Asserts that expression returns [`Poll::Ready(T)`] variant.
///
/// This macro is available for Rust 1.36+.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_ready!`] for assertions that are not enabled in release builds by default.
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
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_ready!`]: ./macro.debug_assert_ready.html
#[macro_export]
macro_rules! assert_ready {
    ($cond:expr,) => {
        $crate::assert_ready!($cond);
    };
    ($cond:expr) => {
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
/// This macro is available for Rust 1.36+.
///
/// Like [`assert_ready!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`Poll::Ready(T)`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_ready!`]: ./macro.assert_ready.html
#[macro_export]
macro_rules! debug_assert_ready {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_ready!($($arg)*); })
}
