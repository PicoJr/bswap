//! Utilities for swapping bytes using patterns and masks.
//!
//! # Quick Start
//!
//! ## Iterators
//!
//! ```
//! use bswap::pattern::{BytePattern, Locality, iter_swap};
//!
//! let pattern = BytePattern::new(0x42, 0xFF); // replace byte by 0x42
//! let locality = Locality::new(2, 1); // replace odd bytes
//! let swap = (pattern, locality); // swap odd bytes with 0x42
//!
//! let source: [u8; 4] = [0x41, 0x41, 0x41, 0x41];
//! let swapped = iter_swap(&swap, &source); // iterator on result
//! let swapped: Vec<u8> = swapped.collect();
//! assert_eq!(swapped, vec!(0x41, 0x42, 0x41, 0x42));
//! ```
//!
//! ## Mutating File-like Data
//!
//! ```
//! use std::io::Cursor;
//! use bswap::pattern::{BytePattern, Locality};
//! use bswap::Swap;
//! use bswap::io::swap_io;
//!
//! // in memory reader (implements `Read`)
//! let mut reader: Cursor<Vec<u8>> = Cursor::new(vec![0x41, 0x42, 0x43, 0x44]);
//! // in memory writer (implements `Write`)
//! let mut writer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
//!
//! let swaps: &[Swap] = &[(BytePattern::new(0x42, 0xFF), Locality::new(2, 0))];
//! let swap = swap_io(&mut reader, &mut writer, swaps);
//! assert!(swap.is_ok());
//! assert_eq!(swap.unwrap(), 4); // 4 bytes written
//! assert_eq!(writer.into_inner(), vec![0x42, 0x42, 0x42, 0x44])
//! ```

use crate::pattern::{BytePattern, Locality};

/// Swap: `(Pattern, Locality)`
pub type Swap = (BytePattern, Locality);

/// default buffer size for io: 8KB
pub const BUFFER_SIZE: usize = 8000; // 8KB

pub mod io;
pub mod pattern;
