/// Asserts that first expression is less or equal than the second.
///
/// Requires that both expressions be comparable with `<=`.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_le`] for assertions that are not enabled in release builds by default.
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
/// # fn main() {
/// assert_le!(1, 2);
///
/// // With custom messages
/// assert_le!(5, 5, "Expecting that {} is less or equal than {}", 5, 5);
/// # }
/// ```
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// assert_le!(6, 5);  // Will panic
///
/// // With custom messages
/// assert_le!(6, 5, "Not expecting {} to be less or equal than {}", 6, 5);
/// # }
/// ```
///
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_le!`]: ./macro.debug_assert_le.html
#[macro_export]
macro_rules! assert_le {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val <= *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left <= right)`
    left: `{:?}`,
    right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    };
    ($left:expr, $right:expr,) => {
        $crate::assert_le!($left, $right);
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val <= *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left <= right)`
    left: `{:?}`,
    right: `{:?}`: {}"#, &*left_val, &*right_val, ::core::format_args!($($arg)+))
                }
            }
        }
    };
}

/// Asserts that first expression is less or equal than the second in runtime.
///
/// Like [`assert_le!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_le!`]: ./macro.assert_le.html
#[macro_export]
macro_rules! debug_assert_le {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_le!($($arg)*); })
}
