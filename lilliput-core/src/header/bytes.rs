use crate::{binary::Byte, error::Expectation, num::int::CompactWidth as _};

use super::{DecodeHeader, EncodeHeader, Marker};

/// Represents a byte sequence.
///
/// # Binary representation
///
/// ```plain
/// 0b000001XX <INTEGER> [ BYTE, … ]
///   ├────┘├┘  └─ Length  └─ Bytes
///   │     └─ Length width exponent
///   └─ Bytes type
/// ```
///
/// The byte-width of the length value is obtained by:
///
/// ```plain
/// width = 2 ^ exponent
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BytesHeader {
    len_width_exponent: u8,
}

impl BytesHeader {
    const TYPE_BITS: u8 = 0b00000100;

    const LEN_WIDTH_EXPONENT_BITS: u8 = 0b00000011;

    #[inline]
    pub fn new(len_width_exponent: u8) -> Self {
        let len_width_exponent =
            Byte::assert_masked_by(len_width_exponent, Self::LEN_WIDTH_EXPONENT_BITS);

        Self { len_width_exponent }
    }

    #[inline]
    pub fn optimal(len: usize) -> Self {
        let len_width: u8 = match len.compact_width() {
            1 => 1,
            2 => 2,
            3..=4 => 4,
            5..=8 => 8,
            _ => unreachable!(),
        };
        Self {
            len_width_exponent: Self::len_width_exponent(len_width),
        }
    }

    #[inline]
    pub fn exact(_len: usize) -> Self {
        Self {
            len_width_exponent: Self::len_width_exponent(8),
        }
    }

    #[inline]
    pub fn len_width(&self) -> u8 {
        1u8 << self.len_width_exponent
    }

    fn len_width_exponent(len_width: u8) -> u8 {
        debug_assert!(len_width <= 8);
        match len_width {
            1 => 0,
            2 => 1,
            3..=4 => 2,
            5..=8 => 3,
            _ => unreachable!(),
        }
    }
}

impl DecodeHeader for BytesHeader {
    fn decode(byte: u8) -> Result<Self, Expectation<Marker>> {
        Marker::Bytes.validate(byte)?;

        let byte = Byte(byte);

        let len_width_exponent = byte.masked_bits(Self::LEN_WIDTH_EXPONENT_BITS);

        Ok(Self { len_width_exponent })
    }
}

impl EncodeHeader for BytesHeader {
    fn encode(self) -> u8 {
        let mut byte = Byte(Self::TYPE_BITS);

        byte.set_bits_assert_masked_by(self.len_width_exponent, Self::LEN_WIDTH_EXPONENT_BITS);

        byte.0
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::prelude::Arbitrary for BytesHeader {
    type Parameters = ();
    type Strategy = proptest::prelude::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy as _;
        (0..=3_u8).prop_map(Self::new).boxed()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn encode_decode_roundtrip(header in BytesHeader::arbitrary()) {
            let encoded = header.encode();
            let decoded = BytesHeader::decode(encoded).unwrap();

            prop_assert_eq!(&decoded, &header);
        }
    }
}
