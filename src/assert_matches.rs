/// Asserts that expression matches any of the given variants.
///
/// This macro is available for Rust 1.32+.
///
/// It works exactly as [`std::matches!`] macro,
/// except it panics if there is no match.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_matches!`] for assertions that are not enabled in release builds by default.
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
/// let foo = 'f';
/// assert_matches!(foo, 'A'..='Z' | 'a'..='z');
///
/// // With custom messages
/// assert_matches!(foo, 'A'..='Z' | 'a'..='z', "expecting it to be letter: {}", foo);
/// # }
/// ```
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let bar: Option<i32> = None;
/// assert_matches!(bar, Some(x) if x > 2);  // Will panic
/// # }
/// ```
///
/// [`std::matches!`]: https://doc.rust-lang.org/stable/std/macro.matches.html
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_matches!`]: ./macro.debug_assert_matches.html
#[macro_export]
macro_rules! assert_matches {
    ($expression:expr, $( $pattern:pat )|+) => {
        match $expression {
            $( $pattern )|+ => {},
            other => {
                panic!(r#"assertion failed, expression does not match any of the given variants.
    expression: {:?}
    variants: {}"#, other, stringify!($($pattern) |+));
            }
        }
    };
    ($expression:expr, $( $pattern:pat )|+ if $guard: expr) => {
        match $expression {
            $( $pattern )|+ if $guard => {},
            other => {
                panic!(r#"assertion failed, expression does not match any of the given variants.
    expression: {:?}
    variants: {}"#, other, stringify!($($pattern) |+ if $guard));
            }
        }
    };
    ($expression:expr, $( $pattern:pat )|+, $($arg:tt)+) => {
        match $expression {
            $( $pattern )|+ => {},
            other => {
                panic!(r#"assertion failed, expression does not match any of the given variants.
    expression: {:?}
    variants: {}: {}"#, other, stringify!($($pattern) |+), format_args!($($arg)+));
            }
        }
    };
    ($expression:expr, $( $pattern:pat )|+ if $guard: expr, $($arg:tt)+) => {
        match $expression {
            $( $pattern )|+ if $guard => {},
            other => {
                panic!(r#"assertion failed, expression does not match any of the given variants.
    expression: {:?}
    variants: {}: {}"#, other, stringify!($($pattern) |+ if $guard), format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression matches any of the given variants.
///
/// Like [`assert_matches!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert_matches!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`Err(E)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Err
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_matches!`]: ./macro.assert_matches.html
#[macro_export]
macro_rules! debug_assert_matches {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_matches!($($arg)*); })
}
