//! Pattern, Locality and iterator utils (pure)

use crate::{BytePattern, PositionPredicate};

/// Stores Pattern Position Predicate
/// which bytes the pattern eval to given their position.
#[non_exhaustive]
pub struct Predicate {
    /// only match every `periodicity` bytes once the `offset` is reached.
    pub periodicity: usize,
    /// skip `offset` first bytes
    /// only bytes with position >= offset will match
    pub offset: usize,
    /// if present only match at most `limit` times bytes.
    ///
    /// it means byte position must verify `(position - offset) / periodicity < limit`
    ///
    /// `None` means no limit.
    pub limit: Option<usize>,
}

/// BytePattern
/// Given a target byte, the pattern will set bits to corresponding bits from `value`
/// according to `mask`.
#[non_exhaustive]
pub struct Pattern {
    pub value: u8,
    pub mask: u8,
}

impl Pattern {
    /// Creates a new `BytePattern` with provided `value`.
    ///
    /// `mask`: 0xFF
    ///
    /// ```
    /// # use bswp::pattern::Pattern;
    /// let byte_pattern = Pattern::new(0xFF).with_mask(0xF0); // replace first 4 bits with 0xF
    /// ```
    pub fn new(value: u8) -> Self {
        Pattern { value, mask: 0xFF }
    }

    pub fn with_mask(mut self, mask: u8) -> Self {
        self.mask = mask;
        self
    }
}

impl BytePattern for Pattern {
    /// Returns the value with current pattern applied.
    ///
    /// ```
    /// use bswp::BytePattern;
    /// use bswp::pattern::Pattern;
    /// let byte_pattern = Pattern::new(0xFF).with_mask(0xF0);
    /// assert_eq!(byte_pattern.eval(0x00), 0xF0);
    /// let byte_pattern = Pattern::new(0b10101111).with_mask(0b10011010);
    /// assert_eq!(byte_pattern.eval(0b00000000), 0b10001010);
    /// ```
    fn eval(&self, value: u8) -> u8 {
        (self.mask & self.value) | (!self.mask & value)
    }
}

impl Default for Predicate {
    /// Creates a default `Locality`
    ///
    /// `periodicity`: `1` (every bytes)
    /// `offset`: `0` (starting from byte at position `0`)
    /// `limit`: `None` (
    fn default() -> Self {
        Predicate {
            periodicity: 1,
            offset: 0,
            limit: None,
        }
    }
}

impl Predicate {
    /// Creates a new `Locality` with default `periodicity`, `offest` and `limit`.
    ///
    /// returns `Locality {periodicity: 1, offset: 0, limit: None}`
    ///
    /// ```
    /// # use bswp::pattern::Predicate;
    /// let locality = Predicate::new(); // matches every bytes
    /// ```
    pub fn new() -> Predicate {
        Predicate::default()
    }

    /// Sets the `periodicity`.
    ///
    /// **Default**: `1`
    pub fn with_periodicity(mut self, periodicity: usize) -> Self {
        self.periodicity = periodicity;
        self
    }

    /// Sets the `offset`.
    ///
    /// **Default**: `0`
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Sets the `limit`.
    ///
    /// **Default**: `None`
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the `limit` to `None` (Default).
    pub fn with_no_limit(mut self) -> Self {
        self.limit = None;
        self
    }
}

impl PositionPredicate for Predicate {
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
    fn eval(&self, position: usize) -> bool {
        (position >= self.offset)
            && self.limit.map_or(true, |limit| {
                ((position - self.offset) / self.periodicity) < limit
            })
            && ((position - self.offset) % self.periodicity) == 0
    }
}

/// Returns an iterator on swapped bytes from `source`.
///
/// ```
/// use bswp::pattern::{Predicate, Pattern, swap_iter};
/// let predicate = Predicate::new().with_periodicity(2).with_offset(1);
/// let pattern = Pattern::new(0x42).with_mask(0xFF);
/// let swaps = &[(pattern, predicate)];
///
/// let source: [u8; 4] = [0x41, 0x41, 0x41, 0x41];
/// let swapped = swap_iter(&source, swaps);
/// let swapped: Vec<u8> = swapped.collect();
/// assert_eq!(swapped, vec!(0x41, 0x42, 0x41, 0x42));
/// ```
pub fn swap_iter<'a, P: BytePattern, Q: PositionPredicate>(
    source: &'a [u8],
    swaps: &'a [(P, Q)],
) -> impl Iterator<Item = u8> + 'a {
    source.iter().enumerate().map(move |(position, &e)| {
        swaps.iter().fold(e, |value, (pattern, predicate)| {
            if predicate.eval(position) {
                pattern.eval(value)
            } else {
                value
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use crate::pattern::{swap_iter, Pattern, Predicate};
    use crate::{BytePattern, PositionPredicate};

    #[test]
    fn test_swap() {
        let pattern = Pattern::new(0xFF).with_mask(0x0F);
        assert_eq!(pattern.eval(0x0), 0x0F);
        let pattern = Pattern::new(0xFF).with_mask(0xF0);
        assert_eq!(pattern.eval(0x0), 0xF0);
    }

    #[test]
    fn test_eval_2_3_none() {
        let locality = Predicate::new().with_periodicity(2).with_offset(3);
        let unexpected_vec: Vec<usize> = vec![0, 1, 2, 4, 6, 8];
        let expected_vec: Vec<usize> = vec![3, 5, 7, 9];
        for unexpected in unexpected_vec {
            assert!(!locality.eval(unexpected));
        }
        for expected in expected_vec {
            assert!(locality.eval(expected));
        }
    }

    #[test]
    fn test_eval_2_3_2() {
        let locality = Predicate::new()
            .with_periodicity(2)
            .with_offset(3)
            .with_limit(2);
        let unexpected_vec: Vec<usize> = vec![0, 1, 2, 4, 6, 7, 8, 9]; // 7, 9 unexpected because of limit
        let expected_vec: Vec<usize> = vec![3, 5];
        for unexpected in unexpected_vec {
            assert!(!locality.eval(unexpected), "{} is unexpected", unexpected);
        }
        for expected in expected_vec {
            assert!(locality.eval(expected));
        }
    }

    #[test]
    fn test_eval_2_1_none() {
        let locality = Predicate::new().with_periodicity(2).with_offset(3);
        let unexpected_vec: Vec<usize> = vec![0, 2, 4, 6, 8];
        let expected_vec: Vec<usize> = vec![3, 5, 7, 9];
        for unexpected in unexpected_vec {
            assert!(!locality.eval(unexpected));
        }
        for expected in expected_vec {
            assert!(locality.eval(expected));
        }
    }

    #[test]
    fn test_iter_swap() {
        let predicate = Predicate::new().with_periodicity(2).with_offset(1);
        let pattern = Pattern::new(0x42).with_mask(0xFF);
        let swaps = &[(pattern, predicate)];

        let source: [u8; 4] = [0x41, 0x41, 0x41, 0x41];
        let swapped = swap_iter(&source, swaps);
        let swapped: Vec<u8> = swapped.collect();
        assert_eq!(swapped, vec!(0x41, 0x42, 0x41, 0x42));
    }
}
