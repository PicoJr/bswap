[![bswp crate](https://img.shields.io/crates/v/bswp.svg)](https://crates.io/crates/bswp)
[![bswp documentation](https://docs.rs/bswp/badge.svg)](https://docs.rs/bswp)
[![GitHub license](https://img.shields.io/github/license/PicoJr/bswp)](https://github.com/PicoJr/bswp/blob/master/LICENSE)

|Branch|Status|
|------|------|
|[master](https://github.com/PicoJr/bswp/tree/master)|![Build Status](https://github.com/PicoJr/bswp/workflows/Rust/badge.svg?branch=master)|

# BSWP

Rust Byte Swap lib. 

Swap bytes using patterns and masks.

## Minimum Supported Rust Version (MSRV)

`bswp` requires Rust [1.40.0](https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html).

## Usage

### Iterators

```rust
use bswp::pattern::{Pattern, Predicate, swap_iter};

let pattern = Pattern::new(0x42).with_mask(0xFF); // replace byte by 0x42
let predicate = Predicate::new().with_periodicity(2).with_offset(1); // replace odd bytes
let swaps = &[(pattern, predicate)];

let source: [u8; 4] = [0x41, 0x41, 0x41, 0x41];
let swapped = swap_iter(&source, swaps); // iterator on result
let swapped: Vec<u8> = swapped.collect();
assert_eq!(swapped, vec!(0x41, 0x42, 0x41, 0x42));
```

### Mutating File-like Data

```rust
use std::io::Cursor;
use bswp::pattern::{Pattern, Predicate};
use bswp::io::swap_io;

// in memory reader (implements `Read`)
let mut reader: Cursor<Vec<u8>> = Cursor::new(vec![0x41, 0x42, 0x43, 0x44]);
// in memory writer (implements `Write`)
let mut writer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

let swaps: &[(Pattern, Predicate)] = &[(Pattern::new(0x42).with_mask(0xFF), Predicate::new().with_periodicity(2).with_offset(0))];
let swap = swap_io(&mut reader, &mut writer, swaps);
assert!(swap.is_ok());
assert_eq!(swap.unwrap(), 4); // 4 bytes written
assert_eq!(writer.into_inner(), vec![0x42, 0x42, 0x42, 0x44])
```

## Changelog

Please see the [CHANGELOG](CHANGELOG.md) for a release history.
