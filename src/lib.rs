//! Better assertion macros.

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
mod assert_ready_err;
#[cfg(has_task_poll)]
mod assert_ready_ok;
