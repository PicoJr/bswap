//! Byte swap IO utils (mut)

use crate::{Swap, BUFFER_SIZE};
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
/// use bswp::pattern::{BytePattern, Locality};
/// use bswp::Swap;
/// use bswp::io::swap_io;
///
/// // in memory reader (implements `Read`)
/// let mut reader: Cursor<Vec<u8>> = Cursor::new(vec![0x41, 0x42, 0x43, 0x44]);
/// // in memory writer (implements `Write`)
/// let mut writer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
///
/// let swaps: &[Swap] = &[(BytePattern::new(0x42, 0xFF), Locality::new(2, 0))];
/// let swap = swap_io(&mut reader, &mut writer, swaps);
/// assert!(swap.is_ok());
/// assert_eq!(swap.unwrap(), 4); // 4 bytes written
/// assert_eq!(writer.into_inner(), vec![0x42, 0x42, 0x42, 0x44])
/// ```
pub fn swap_io(
    reader: &mut dyn Read,
    writer: &mut dyn Write,
    swaps: &[Swap],
) -> Result<usize, std::io::Error> {
    let mut position: usize = 0;
    let mut buffer = [0; BUFFER_SIZE];

    while let Ok(size) = reader.read(&mut buffer) {
        if size == 0 {
            break; // finished
        }
        for (position_in_buffer, item) in buffer.iter_mut().enumerate().take(size) {
            let byte_position = position + position_in_buffer; // position relative to reader start
            for (pattern, locality) in swaps {
                if locality.applies(byte_position) {
                    *item = pattern.swap(*item);
                }
            }
        }
        position += size;
        writer.write_all(&buffer[..size])?;
    }
    Ok(position)
}
