#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct Byte(pub u8);

impl Byte {
    /// Returns `true`, if `self` contains only bits from `mask`, otherwise `false`.
    #[inline(always)]
    pub(crate) fn is_masked_by(self, mask: u8) -> bool {
        self.0 & !mask == 0b0
    }

    /// Returns `true`, if `self` contains all bits from `mask`, or `mask` itself is empty, otherwise `false`.
    #[inline(always)]
    pub(crate) fn contains_all_bits_of(self, mask: u8) -> bool {
        (self.0 & mask == mask) || mask == 0b0
    }

    /// Returns `true`, if `self` contains any bits from `mask`, or `mask` itself is empty, otherwise `false`.
    #[inline(always)]
    pub(crate) fn contains_any_bits_of(self, mask: u8) -> bool {
        (self.0 & mask != 0b0) || mask == 0b0
    }

    /// Returns `true`, if `self` contains no bits from `mask`.
    #[inline(always)]
    pub(crate) fn contains_no_bits_of(self, mask: u8) -> bool {
        self.0 & mask == 0b0
    }

    /// Performs a logical `self &= mask`, clearing all bits not found in `mask`.
    #[inline(always)]
    pub(crate) fn mask_bits(&mut self, mask: u8) {
        self.0 &= mask;
    }

    /// Performs a logical `self \= mask`, setting all bits found in `mask`.
    #[inline(always)]
    pub(crate) fn set_bits(&mut self, mask: u8) {
        self.0 |= mask;
    }

    /// Performs a logical `self &= !mask`, clearing all bits found in `mask`.
    #[inline(always)]
    pub(crate) fn clear_bits(&mut self, mask: u8) {
        self.0 &= !mask;
    }

    /// Conditionally sets bits (branch-less).
    #[inline(always)]
    pub(crate) fn set_bits_if(&mut self, mask: u8, condition: bool) {
        self.set_bits(Self::mask_if(mask, condition));
    }

    /// Conditionally clears bits (branch-less).
    #[inline(always)]
    pub(crate) fn clear_bits_if(&mut self, mask: u8, condition: bool) {
        self.clear_bits(Self::mask_if(mask, condition));
    }

    #[inline(always)]
    fn mask_if(mask: u8, condition: bool) -> u8 {
        mask & Self::mask_all_if(condition)
    }

    #[inline(always)]
    fn mask_all_if(condition: bool) -> u8 {
        !(condition as u8).wrapping_sub(1)
    }
}

impl std::fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{:0>2x}", self.0)
    }
}

impl std::fmt::Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0b")?;
        }
        write!(f, "{:08b}", self.0)
    }
}

impl std::fmt::LowerHex for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{:0>2x}", self.0)
    }
}

impl std::fmt::UpperHex for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{:0>2X}", self.0)
    }
}

impl std::fmt::Octal for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0o")?;
        }
        write!(f, "{:0>3o}", self.0)
    }
}

impl std::fmt::Binary for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0b")?;
        }
        write!(f, "{:08b}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_masked_by() {
        let byte = Byte(0b01010101);

        assert!(!byte.is_masked_by(0b00000000));
        assert!(!byte.is_masked_by(0b01010000));
        assert!(!byte.is_masked_by(0b00001111));
        assert!(!byte.is_masked_by(0b10101010));

        assert!(byte.is_masked_by(0b01010101));
        assert!(byte.is_masked_by(0b01011111));
        assert!(byte.is_masked_by(0b11111111));
    }

    #[test]
    fn contains_all_bits_of() {
        let byte = Byte(0b01010101);

        assert!(!byte.contains_all_bits_of(0b10101010));
        assert!(!byte.contains_all_bits_of(0b11110101));
        assert!(!byte.contains_all_bits_of(0b11111111));

        assert!(byte.contains_all_bits_of(0b00000000));
        assert!(byte.contains_all_bits_of(0b00000101));
        assert!(byte.contains_all_bits_of(0b01010101));
    }

    #[test]
    fn contains_any_bits_of() {
        let byte = Byte(0b01010101);

        assert!(!byte.contains_any_bits_of(0b10100000));
        assert!(!byte.contains_any_bits_of(0b10101010));

        assert!(byte.contains_any_bits_of(0b00000000));
        assert!(byte.contains_any_bits_of(0b00000101));
        assert!(byte.contains_any_bits_of(0b00001111));
        assert!(byte.contains_any_bits_of(0b11111111));
    }

    #[test]
    fn contains_no_bits_of() {
        let byte = Byte(0b01010101);

        assert!(!byte.contains_no_bits_of(0b01010101));
        assert!(!byte.contains_no_bits_of(0b01011111));
        assert!(!byte.contains_no_bits_of(0b11111111));

        assert!(byte.contains_no_bits_of(0b00000000));
        assert!(byte.contains_no_bits_of(0b10101010));
    }

    #[test]
    fn mask_bits() {
        let mut byte = Byte(0b11111111);

        byte.mask_bits(0b11111111);
        assert_eq!(byte, Byte(0b11111111));

        byte.mask_bits(0b11110000);
        assert_eq!(byte, Byte(0b11110000));

        byte.mask_bits(0b01010101);
        assert_eq!(byte, Byte(0b01010000));

        byte.mask_bits(0b00000000);
        assert_eq!(byte, Byte(0b00000000));
    }

    #[test]
    fn set_bits() {
        let mut byte = Byte(0b0000000);

        byte.set_bits(0b00000000);
        assert_eq!(byte, Byte(0b00000000));

        byte.set_bits(0b11110000);
        assert_eq!(byte, Byte(0b11110000));

        byte.set_bits(0b01010101);
        assert_eq!(byte, Byte(0b11110101));

        byte.set_bits(0b11111111);
        assert_eq!(byte, Byte(0b11111111));
    }

    #[test]
    fn clear_bits() {
        let mut byte = Byte(0b11111111);

        byte.clear_bits(0b00000000);
        assert_eq!(byte, Byte(0b11111111));

        byte.clear_bits(0b11110000);
        assert_eq!(byte, Byte(0b00001111));

        byte.clear_bits(0b01010101);
        assert_eq!(byte, Byte(0b00001010));

        byte.clear_bits(0b11111111);
        assert_eq!(byte, Byte(0b00000000));
    }

    #[test]
    fn set_bits_if() {
        let mut byte = Byte(0b0000000);

        byte.set_bits_if(0b00000000, false);
        assert_eq!(byte, Byte(0b00000000));
        byte.set_bits_if(0b00000000, true);
        assert_eq!(byte, Byte(0b00000000));

        byte.set_bits_if(0b11110000, false);
        assert_eq!(byte, Byte(0b0000000));
        byte.set_bits_if(0b11110000, true);
        assert_eq!(byte, Byte(0b11110000));

        byte.set_bits_if(0b01010101, false);
        assert_eq!(byte, Byte(0b11110000));
        byte.set_bits_if(0b01010101, true);
        assert_eq!(byte, Byte(0b11110101));

        byte.set_bits_if(0b11111111, false);
        assert_eq!(byte, Byte(0b11110101));
        byte.set_bits_if(0b11111111, true);
        assert_eq!(byte, Byte(0b11111111));
    }

    #[test]
    fn clear_bits_if() {
        let mut byte = Byte(0b11111111);

        byte.clear_bits_if(0b00000000, false);
        assert_eq!(byte, Byte(0b11111111));
        byte.clear_bits_if(0b00000000, true);
        assert_eq!(byte, Byte(0b11111111));

        byte.clear_bits_if(0b11110000, false);
        assert_eq!(byte, Byte(0b11111111));
        byte.clear_bits_if(0b11110000, true);
        assert_eq!(byte, Byte(0b00001111));

        byte.clear_bits_if(0b01010101, false);
        assert_eq!(byte, Byte(0b00001111));
        byte.clear_bits_if(0b01010101, true);
        assert_eq!(byte, Byte(0b00001010));

        byte.clear_bits_if(0b11111111, false);
        assert_eq!(byte, Byte(0b00001010));
        byte.clear_bits_if(0b11111111, true);
        assert_eq!(byte, Byte(0b00000000));
    }

    #[test]
    fn display() {
        let byte = Byte(42);

        assert_eq!(format!("{byte}"), "2a");
    }

    #[test]
    fn debug() {
        let byte = Byte(42);

        assert_eq!(format!("{byte:?}"), "00101010");
        assert_eq!(format!("{byte:#?}"), "0b00101010");
    }

    #[test]
    fn lower_hex() {
        let byte = Byte(42);

        assert_eq!(format!("{byte:x}"), "2a");
        assert_eq!(format!("{byte:#x}"), "0x2a");
    }

    #[test]
    fn upper_hex() {
        let byte = Byte(42);

        assert_eq!(format!("{byte:X}"), "2A");
        assert_eq!(format!("{byte:#X}"), "0x2A");
    }

    #[test]
    fn octal() {
        let byte = Byte(42);

        assert_eq!(format!("{byte:o}"), "052");
        assert_eq!(format!("{byte:#o}"), "0o052");
    }

    #[test]
    fn binary() {
        let byte = Byte(42);

        assert_eq!(format!("{byte:b}"), "00101010");
        assert_eq!(format!("{byte:#b}"), "0b00101010");
    }
}
