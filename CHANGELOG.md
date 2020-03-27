# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0](https://crates.io/crates/bswp/0.1.0) Mar 27, 2020

### Added

* `Pattern` and `Predicate` builders
* non exhaustive on `Pattern` and `Predicate` structs
* `Pattern` and `Predicate` structs fields now public
* `BytePattern` and `PositionPredicate` traits
* `iter_swap` renamed to `swap_iter`
* generic `swap_iter` and `swap_io`

### Breaking Changes

**Everything** was renamed and modified `^^'`

## [0.1.0](https://crates.io/crates/bswp/0.1.0) Mar 24, 2020

### Added

* `pattern::BytePattern`
* `pattern::Locality`
* `pattern::iter_swap`
* `io::swap_io`
