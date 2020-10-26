/// Asserts that expression returns [`Ok(T)`] variant.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_ok!`] for assertions that are not enabled in release builds by default.
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
/// let res: Result<i32, ()> = Ok(1);
///
/// assert_ok!(res);
///
/// // With custom messages
/// assert_ok!(res, "Everything is good with {:?}", res);
/// # }
/// ```
///
/// Value of `T` type from `Ok(T)` will be returned from the macro call:
///
/// ```rust
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let res: Result<i32, ()> = Ok(1);
///
/// let value = assert_ok!(res);
/// assert_eq!(value, 1);
/// # }
/// ```
///
/// `Err(..)` variant will cause panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claim;
/// # fn main() {
/// let res = Err(());
///
/// assert_ok!(res);  // Will panic
/// # }
/// ```
///
/// [`Ok(T)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Ok
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_ok!`]: ./macro.debug_assert_ok.html
#[macro_export]
macro_rules! assert_ok {
    ($cond:expr,) => {
        $crate::assert_ok!($cond);
    };
    ($cond:expr) => {
        match $cond {
            Ok(t) => t,
            Err(e) => {
                panic!("assertion failed, expected Ok(..), got Err({:?})", e);
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            Ok(t) => t,
            Err(e) => {
                panic!("assertion failed, expected Ok(..), got Err({:?}): {}", e, format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns [`Ok(T)`] variant in runtime.
///
/// Like [`assert_ok!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`Ok(T)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Ok
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_ok!`]: ./macro.assert_ok.html
#[macro_export]
macro_rules! debug_assert_ok {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_ok!($($arg)*); })
}

#[cfg(test)]
#[cfg(not(has_private_in_public_issue))]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed, expected Ok(..), got Err(())")]
    fn default_panic_message() {
        let res = Err(());
        assert_ok!(res);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed, expected Ok(..), got Err(()): Everything is good with Err(())"
    )]
    fn custom_panic_message() {
        let res = Err(());
        assert_ok!(res, "Everything is good with {:?}", res);
    }

    #[test]
    fn does_not_require_ok_debug() {
        enum Foo {
            Bar,
        }

        let res: Result<Foo, ()> = Ok(Foo::Bar);
        let _ = assert_ok!(res);
    }
}
