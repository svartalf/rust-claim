/// Asserts that first expression is less than the second.
///
/// Requires that both exceptions be comparable with `<`.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// assert_lt!(1, 2);
/// # }
/// ```
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// assert_lt!(5, 5);  // Will panic
/// assert_le!(6, 5);
/// # }
/// ```
#[macro_export]
macro_rules! assert_lt {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val < *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left < right)`
    left: `{:?}`,
    right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    };
    ($left:expr, $right:expr,) => {
        $crate::assert_lt!($left, $right);
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val < *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left < right)`
    left: `{:?}`,
    right: `{:?}`: {}"#, &*left_val, &*right_val, ::core::format_args!($($arg)+))
                }
            }
        }
    };
}

/// Asserts that first expression is less or equal than the second in runtime.
#[macro_export]
macro_rules! debug_assert_lt {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_lt!($($arg)*); })
}
