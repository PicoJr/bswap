[![bswp crate](https://img.shields.io/crates/v/bswp.svg)](https://crates.io/crates/bswp)
[![bswp documentation](https://docs.rs/bswp/badge.svg)](https://docs.rs/bswp)
[![GitHub license](https://img.shields.io/github/license/PicoJr/bswp)](https://github.com/PicoJr/bswp/blob/master/LICENSE)

|Branch|Status|
|------|------|
|[master](https://github.com/PicoJr/bswp/tree/master)|![Build Status](https://github.com/PicoJr/bswp/workflows/Rust/badge.svg?branch=master)|

# BSWP

Rust Byte Swap lib.

Swap bytes using patterns and masks.

## Usage

### Iterators

``` rust
use bswp::pattern::{BytePattern, Locality, iter_swap};

let pattern = BytePattern::new(0x42, 0xFF); // replace byte by 0x42
let locality = Locality::new(2, 1); // replace odd bytes
let swap = (pattern, locality); // swap odd bytes with 0x42

let source: [u8; 4] = [0x41, 0x41, 0x41, 0x41];
let swapped = iter_swap(&swap, &source); // iterator on result
let swapped: Vec<u8> = swapped.collect();
assert_eq!(swapped, vec!(0x41, 0x42, 0x41, 0x42));
```

### Mutating File-like Data

``` rust
use std::io::Cursor;
use bswp::pattern::{BytePattern, Locality};
use bswp::Swap;
use bswp::io::swap_io;

// in memory reader (implements `Read`)
let mut reader: Cursor<Vec<u8>> = Cursor::new(vec![0x41, 0x42, 0x43, 0x44]);
// in memory writer (implements `Write`)
let mut writer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

let swaps: &[Swap] = &[(BytePattern::new(0x42, 0xFF), Locality::new(2, 0))];
let swap = swap_io(&mut reader, &mut writer, swaps);
assert!(swap.is_ok());
assert_eq!(swap.unwrap(), 4); // 4 bytes written
assert_eq!(writer.into_inner(), vec![0x42, 0x42, 0x42, 0x44])
```

## Changelog

Please see the [CHANGELOG](CHANGELOG.md) for a release history.
