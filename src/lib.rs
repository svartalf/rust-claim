#![cfg_attr(rustc_1_6, no_std)]
#![doc(html_root_url = "https://docs.rs/claim/0.5.0")]
#![allow(unknown_lints, unused_extern_crates)]
#![forbid(
    rust_2018_idioms,
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
    broken_intra_doc_links,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_results,
    unsafe_code
)]

//! Assertion macros.
//!
//! This crate provides additional assert macros to make testing a bit easier.
//!
//! ## `#![no_std]` support
//!
//! `claim` can be used in a `no-std` environments too.
//!
//! ## Available macros
//!
//! Note that same to [`core`]/[`std`] macros,
//! all macros in this crate has the [`debug_*`](#macros) counterparts.
//!
//! ### Comparison
//!
//! Assertions similar to [`assert_eq`] or [`assert_ne`]:
//!
//! * [`assert_ge`]
//! * [`assert_gt`]
//! * [`assert_le`]
//! * [`assert_lt`]
//!
//! ### Matching
//!
//! * [`assert_matches`]
//!
//! ### `Result` macros
//!
//! Assertions for [`Result`] variants:
//!
//! * [`assert_ok`]
//! * [`assert_err`]
//! * [`assert_ok_eq`]
//!
//! ### `Option` macros
//!
//! Assertions for [`Option`] variants:
//!
//! * [`assert_some`]
//! * [`assert_none`]
//! * [`assert_some_eq`]
//!
//! ### `Poll` macros
//!
//! Assertions for [`Poll`] variants:
//!
//! * [`assert_ready`]
//! * [`assert_ready_ok`]
//! * [`assert_ready_err`]
//! * [`assert_ready_pending`]
//! * [`assert_ready_eq`]
//!
//! [`core`]: https://doc.rust-lang.org/stable/core/#macros
//! [`std`]: https://doc.rust-lang.org/stable/std/#macros
//! [`Option`]: https://doc.rust-lang.org/core/option/enum.Option.html
//! [`Result`]: https://doc.rust-lang.org/core/result/enum.Result.html
//! [`Poll`]: https://doc.rust-lang.org/core/task/enum.Poll.html
//! [`assert_eq`]: https://doc.rust-lang.org/std/macro.assert_eq.html
//! [`assert_ne`]: https://doc.rust-lang.org/std/macro.assert_ne.html
//! [`assert_ge`]: ./macro.assert_ge.html
//! [`assert_gt`]: ./macro.assert_gt.html
//! [`assert_le`]: ./macro.assert_le.html
//! [`assert_lt`]: ./macro.assert_lt.html
//! [`assert_some`]: ./macro.assert_some.html
//! [`assert_none`]: ./macro.assert_none.html
//! [`assert_some_eq`]: ./macro.assert_some_eq.html
//! [`assert_ok`]: ./macro.assert_ok.html
//! [`assert_err`]: ./macro.assert_err.html
//! [`assert_ok_eq`]: ./macro.assert_ok_eq.html
//! [`assert_ready`]: ./macro.assert_ready.html
//! [`assert_ready_ok`]: ./macro.assert_ready_ok.html
//! [`assert_ready_err`]: ./macro.assert_ready_err.html
//! [`assert_ready_pending`]: ./macro.assert_ready_pending.html
//! [`assert_ready_eq`]: ./macro.assert_ready_eq.html
//! [`assert_matches`]: ./macro.assert_matches.html

mod assert_err;
mod assert_ge;
mod assert_gt;
mod assert_le;
mod assert_lt;
mod assert_none;
mod assert_ok;
mod assert_ok_eq;
mod assert_some;
mod assert_some_eq;

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

#[cfg(rustc_1_26)]
mod assert_matches;
