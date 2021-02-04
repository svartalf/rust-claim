# claim

> Missing assertion macros for Rust

[![Latest Version](https://img.shields.io/crates/v/claim.svg)](https://crates.io/crates/claim)
[![Latest Version](https://docs.rs/claim/badge.svg)](https://docs.rs/claim)
[![Build Status](https://github.com/svartalf/rust-claim/workflows/Continuous%20integration/badge.svg)](https://github.com/svartalf/rust-claim/actions)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)
![no-std compatible](https://img.shields.io/badge/no--std-compatible-brightgreen)
![Version compatibility](https://img.shields.io/badge/Rust-1.0%2B-blue)

This crate provides assertion macros that are missing in the Rust `libcore` / `libstd`:

 * Comparison: `assert_ge`, `assert_gt`, `assert_le`, and `assert_lt`
 * Matching: `assert_matches`
 * `Result`: `assert_ok`, `assert_err`, and `assert_ok_eq`
 * `Option`: `assert_some`, `assert_none`, and `assert_some_eq`
 * `Poll`: `assert_pending`, `assert_ready`, `assert_ready_ok`, `assert_ready_err`, and `assert_ready_eq`

## Installation

Add the following to your `Cargo.toml` manifest
to use this crate for tests, examples and benchmarks:

```toml
[dev-dependencies]
claim = "0.5"
```

## Usage

Check out the [documentation](https://docs.rs/claim) for available macros and examples.

## License

Licensed under either of [Apache License 2.0](https://github.com/svartalf/rust-claim/blob/master/LICENSE-APACHE)
or [MIT license](https://github.com/svartalf/rust-claim/blob/master/LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
