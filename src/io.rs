//! Byte swap IO utils (mut)

use crate::{BytePattern, PositionPredicate, BUFFER_SIZE};
use std::io::{Read, Write};

/// For each byte in `reader` compute pattern and write result to `writer`.
///
/// Returns number of bytes read from `reader` and written to `writer`.
///
/// Internally it relies on a `BUFFER_SIZE` buffer between `reader` and `writer`.
///
/// Please note that `swap_io`:
/// * resets neither of `reader`/`writer` cursors before reading from/writing to it.
/// * resets neither of `reader`/`writer` cursors after reading from/writing to it.
///
/// ```
/// use std::io::Cursor;
/// use bswp::pattern::{Pattern, Predicate};
/// use bswp::io::swap_io;
///
/// // in memory reader (implements `Read`)
/// let mut reader: Cursor<Vec<u8>> = Cursor::new(vec![0x41, 0x42, 0x43, 0x44]);
/// // in memory writer (implements `Write`)
/// let mut writer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
///
/// let swaps: &[(Pattern, Predicate)] = &[(Pattern::new(0x42).with_mask(0xFF), Predicate::new().with_periodicity(2).with_offset(0))];
/// let swap = swap_io(&mut reader, &mut writer, swaps);
/// assert!(swap.is_ok());
/// assert_eq!(swap.unwrap(), 4); // 4 bytes written
/// assert_eq!(writer.into_inner(), vec![0x42, 0x42, 0x42, 0x44])
/// ```
pub fn swap_io<P: BytePattern, Q: PositionPredicate>(
    reader: &mut dyn Read,
    writer: &mut dyn Write,
    swaps: &[(P, Q)],
) -> Result<usize, std::io::Error> {
    let mut position: usize = 0;
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let size = reader.read(&mut buffer)?;
        if size == 0 {
            break; // finished
        }
        for (position_in_buffer, item) in buffer.iter_mut().enumerate().take(size) {
            let byte_position = position + position_in_buffer; // position relative to reader start
            for (pattern, predicate) in swaps {
                if predicate.eval(byte_position) {
                    *item = pattern.eval(*item);
                }
            }
        }
        position += size;
        writer.write_all(&buffer[..size])?;
    }
    Ok(position)
}
