//! Assertion macros.
//!
//! This crate provides additional assert macros to make testing a bit easier.
//!
//! Same to [`std`] macros, all macros in this crate has the [`debug_*`](#macros) counterparts,
//! which are not enabled in release builds by default.
//!
//! ## Comparison
//!
//! Rich comparision, similar to [`assert_eq`]:
//!
//! | Macro         | Operator |
//! | ------------- | :------: |
//! | [`assert_ge`] | `>=`     |
//! | [`assert_gt`] | `>`      |
//! | [`assert_le`] | `<=`     |
//! | [`assert_lt`] | `<`      |
//!
//! ## `Result` macros
//!
//! Assertions for [`Result`] variants:
//!
//! * [`assert_ok`]
//! * [`assert_err`]
//!
//! ## `Option` macros
//!
//! Assertions for [`Option`] variants:
//!
//! * [`assert_some`]
//! * [`assert_none`]
//!
//! ## `Poll` macros
//!
//! Assertions for [`Poll`] variants:
//!
//! * [`assert_ready`]
//! * [`assert_ready_ok`]
//! * [`assert_ready_err`]
//! * [`assert_ready_pending`]
//! * [`assert_ready_eq`]
//!
//! [`std`]: https://doc.rust-lang.org/stable/std/#macros
//! [`Option`]: https://doc.rust-lang.org/core/option/enum.Option.html
//! [`Result`]: https://doc.rust-lang.org/core/result/enum.Result.html
//! [`Poll`]: https://doc.rust-lang.org/core/task/enum.Poll.html
//! [`assert_ge`]: ./macro.assert_ge.html
//! [`assert_gt`]: ./macro.assert_gt.html
//! [`assert_le`]: ./macro.assert_le.html
//! [`assert_lt`]: ./macro.assert_lt.html
//! [`assert_some`]: ./macro.assert_some.html
//! [`assert_none`]: ./macro.assert_none.html
//! [`assert_ok`]: ./macro.assert_ok.html
//! [`assert_err`]: ./macro.assert_err.html
//! [`assert_ready`]: ./macro.assert_ready.html
//! [`assert_ready_ok`]: ./macro.assert_ready_ok.html
//! [`assert_ready_err`]: ./macro.assert_ready_err.html
//! [`assert_ready_pending`]: ./macro.assert_ready_pending.html
//! [`assert_ready_eq`]: ./macro.assert_ready_eq.html

#![doc(html_root_url = "https://docs.rs/claim/0.1.0")]
#![no_std]
#![allow(unknown_lints)]
#![forbid(
    unused,
    unused_imports,
    unused_features,
    bare_trait_objects,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    dead_code,
    deprecated,
    intra_doc_link_resolution_failure,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    unsafe_code
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod assert_err;
mod assert_ge;
mod assert_gt;
mod assert_le;
mod assert_lt;
mod assert_none;
mod assert_ok;
mod assert_some;

#[cfg(has_task_poll)]
mod assert_pending;
#[cfg(has_task_poll)]
mod assert_ready;
#[cfg(has_task_poll)]
mod assert_ready_eq;
#[cfg(has_task_poll)]
mod assert_ready_err;
#[cfg(has_task_poll)]
mod assert_ready_ok;