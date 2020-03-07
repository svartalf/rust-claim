/// Asserts that first expression is greater or equal than the second.
///
/// Requires that both exceptions be comparable with `>=`.
///
/// ## Examples
///
/// ```rust
/// claim::assert_ge!(2, 1);
/// claim::assert_ge!(5, 5);
/// ```
///
/// ```rust,should_panic
/// claim::assert_ge!(5, 6);  // Will panic
/// ```
#[macro_export]
macro_rules! assert_ge {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val >= *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left >= right)`
    left: `{:?}`,
    right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    };
    ($left:expr, $right:expr,) => {
        $crate::assert_ge!($left, $right);
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val >= *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left >= right)`
    left: `{:?}`,
    right: `{:?}`: {}"#, &*left_val, &*right_val, ::core::format_args!($($arg)+))
                }
            }
        }
    };
}

/// Asserts that first expression is greater or equal than the second in runtime.
#[macro_export]
macro_rules! debug_assert_ge {
    ($($arg:tt)*) => (if ::core::cfg!(debug_assertions) { $crate::assert_ge!($($arg)*); })
}
