//! Pattern, Locality and iterator utils (pure)

use crate::Swap;

/// Stores Pattern locality data
/// which bytes the pattern applies to given their position.
pub struct Locality {
    /// only match every `periodicity` bytes once the `offset` is reached.
    periodicity: usize,
    /// skip `offset` first bytes
    /// only bytes with position >= offset will match
    offset: usize,
}

/// BytePattern
/// Given a target byte, the pattern will set bits to corresponding bits from `value`
/// according to `mask`.
pub struct BytePattern {
    value: u8,
    mask: u8,
}

impl BytePattern {
    /// Creates a new `BytePattern` with provided `value` and `mask`.
    ///
    /// ```
    /// # use bswp::pattern::BytePattern;
    /// let byte_pattern = BytePattern::new(0xFF, 0xF0); // replace first 4 bits with 0xF
    /// ```
    pub fn new(value: u8, mask: u8) -> Self {
        BytePattern { value, mask }
    }

    /// Returns the value with current pattern applied.
    ///
    /// ```
    /// # use bswp::pattern::BytePattern;
    /// let byte_pattern = BytePattern::new(0xFF, 0xF0);
    /// assert_eq!(byte_pattern.swap(0x00), 0xF0);
    /// let byte_pattern = BytePattern::new(0b10101111, 0b10011010);
    /// assert_eq!(byte_pattern.swap(0b00000000), 0b10001010);
    /// ```
    pub fn swap(&self, value: u8) -> u8 {
        (self.mask & self.value) | (!self.mask & value)
    }
}

impl Locality {
    /// Creates a new `Locality` with provided `periodicity` and `offset`.
    ///
    /// ```
    /// # use bswp::pattern::Locality;
    /// let locality = Locality::new(2, 3); // every 2 bytes for position >= 3
    /// ```
    pub fn new(periodicity: usize, offset: usize) -> Locality {
        Locality {
            periodicity,
            offset,
        }
    }

    /// Returns `true` if `position` matches locality rules else `false`.
    ///
    /// ```
    /// use bswp::pattern::Locality;
    /// let locality = Locality::new(2, 3);  // every 2 bytes for position >= 3
    /// assert!(!locality.applies(0));
    /// assert!(!locality.applies(1));
    /// assert!(locality.applies(3));
    /// assert!(!locality.applies(4));
    /// assert!(locality.applies(5));
    /// ```
    pub fn applies(&self, position: usize) -> bool {
        (position >= self.offset) && ((position - self.offset) % self.periodicity) == 0
    }
}

/// Returns an iterator on swapped bytes from `source`.
///
/// ```
/// use bswp::pattern::{Locality, BytePattern, iter_swap};
/// let locality = Locality::new(2, 1);
/// let pattern = BytePattern::new(0x42, 0xFF);
/// let swap = (pattern, locality);
///
/// let source: [u8; 4] = [0x41, 0x41, 0x41, 0x41];
/// let swapped = iter_swap(&swap, &source);
/// let swapped: Vec<u8> = swapped.collect();
/// assert_eq!(swapped, vec!(0x41, 0x42, 0x41, 0x42));
/// ```
pub fn iter_swap<'a>(swap: &'a Swap, source: &'a [u8]) -> impl Iterator<Item = u8> + 'a {
    let (pattern, locality) = swap;
    source.iter().enumerate().map(move |(position, &e)| {
        if locality.applies(position) {
            pattern.swap(e)
        } else {
            e
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::pattern::{iter_swap, BytePattern, Locality};

    #[test]
    fn test_swap() {
        let pattern = BytePattern::new(0xFF, 0x0F);
        assert_eq!(pattern.swap(0x0), 0x0F);
        let pattern = BytePattern::new(0xFF, 0xF0);
        assert_eq!(pattern.swap(0x0), 0xF0);
    }

    #[test]
    fn test_applies_2_3() {
        let locality = Locality::new(2, 3);
        let unexpected_vec: Vec<usize> = vec![0, 1, 2, 4, 6, 8];
        let expected_vec: Vec<usize> = vec![3, 5, 7, 9];
        for unexpected in unexpected_vec {
            assert!(!locality.applies(unexpected));
        }
        for expected in expected_vec {
            assert!(locality.applies(expected));
        }
    }

    #[test]
    fn test_applies_2_1() {
        let locality = Locality::new(2, 1);
        let unexpected_vec: Vec<usize> = vec![0, 2, 4, 6, 8];
        let expected_vec: Vec<usize> = vec![3, 5, 7, 9];
        for unexpected in unexpected_vec {
            assert!(!locality.applies(unexpected));
        }
        for expected in expected_vec {
            assert!(locality.applies(expected));
        }
    }

    #[test]
    fn test_iter_swap() {
        let locality = Locality::new(2, 1);
        let pattern = BytePattern::new(0x42, 0xFF);
        let swap = (pattern, locality);

        let source: [u8; 4] = [0x41, 0x41, 0x41, 0x41];
        let swapped = iter_swap(&swap, &source);
        let swapped: Vec<u8> = swapped.collect();
        assert_eq!(swapped, vec!(0x41, 0x42, 0x41, 0x42));
    }
}
