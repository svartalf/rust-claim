# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2021-02-04

### Changed

- Fixed compatibility for Rust versions <= 1.6.0
- Fixed compatibility with Rust version 1.30.0
- MSRV for `assert_matches!` macro was changed from Rust 1.32.0 to 1.26.0

## [0.4.0] - 2020-10-26

### Changed

- Removed `Debug` requirement for `Ok(T)` in `assert_err!` macro
- Removed `Debug` requirement for `Err(e)` in `assert_ok!` macro

## [0.3.1] - 2020-03-13

### Changed

- Minimal required Rust version for `assert_matches!` macro downgraded from the 1.37 to 1.32

## [0.3.0] - 2020-03-13

### Added

- `assert_matches!` macro

## [0.2.0] - 2020-03-09

### Added

- `assert_some_eq!` macro
- `assert_ok_eq!` macro

### Changed

- Ensuring support for older Rust versions
