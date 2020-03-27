//! Utilities for swapping bytes using patterns and masks.
//!
//! # Quick Start
//!
//! ## Iterators
//!
//! ```
//! use bswp::pattern::{Pattern, Predicate, swap_iter};
//!
//! let pattern = Pattern::new(0x42).with_mask(0xFF); // replace byte by 0x42
//! let predicate = Predicate::new().with_periodicity(2).with_offset(1); // replace odd bytes
//! let swaps = &[(pattern, predicate)];
//!
//! let source: [u8; 4] = [0x41, 0x41, 0x41, 0x41];
//! let swapped = swap_iter(&source, swaps); // iterator on result
//! let swapped: Vec<u8> = swapped.collect();
//! assert_eq!(swapped, vec!(0x41, 0x42, 0x41, 0x42));
//! ```
//!
//! ## Mutating File-like Data
//!
//! ```
//! use std::io::Cursor;
//! use bswp::pattern::{Pattern, Predicate};
//! use bswp::io::swap_io;
//!
//! // in memory reader (implements `Read`)
//! let mut reader: Cursor<Vec<u8>> = Cursor::new(vec![0x41, 0x42, 0x43, 0x44]);
//! // in memory writer (implements `Write`)
//! let mut writer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
//!
//! let swaps: &[(Pattern, Predicate)] = &[(Pattern::new(0x42).with_mask(0xFF), Predicate::new().with_periodicity(2).with_offset(0))];
//! let swap = swap_io(&mut reader, &mut writer, swaps);
//! assert!(swap.is_ok());
//! assert_eq!(swap.unwrap(), 4); // 4 bytes written
//! assert_eq!(writer.into_inner(), vec![0x42, 0x42, 0x42, 0x44])
//! ```

/// default buffer size for io: 8KB
pub const BUFFER_SIZE: usize = 8000; // 8KB

/// Predicate on byte position.
pub trait PositionPredicate {
    /// Returns `true` if `position` matches locality rules else `false`.
    ///
    /// ```
    /// use bswp::PositionPredicate;
    /// use bswp::pattern::Predicate;
    /// let locality = Predicate::new().with_periodicity(2).with_offset(3);  // every 2 bytes for position >= 3
    /// assert!(!locality.eval(0));
    /// assert!(!locality.eval(1));
    /// assert!(locality.eval(3));
    /// assert!(!locality.eval(4));
    /// assert!(locality.eval(5));
    /// ```
    fn eval(&self, position: usize) -> bool;
}

/// Pattern on byte.
pub trait BytePattern {
    /// Returns the value with current pattern applied.
    ///
    /// ```
    /// use bswp::pattern::Pattern;
    /// use bswp::BytePattern;
    /// let byte_pattern = Pattern::new(0xFF).with_mask(0xF0);
    /// assert_eq!(byte_pattern.eval(0x00), 0xF0);
    /// let byte_pattern = Pattern::new(0b10101111).with_mask(0b10011010);
    /// assert_eq!(byte_pattern.eval(0b00000000), 0b10001010);
    /// ```
    fn eval(&self, value: u8) -> u8;
}

pub mod io;
pub mod pattern;
