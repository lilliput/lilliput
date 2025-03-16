use crate::binary::{required_bytes_for_prim_int, Byte};

use super::{DecodeHeader, EncodeHeader, HeaderDecodeError, HeaderType};

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
    len_width_exponent: usize,
}

impl BytesHeader {
    const TYPE_BITS: u8 = 0b00000100;

    const LEN_WIDTH_EXPONENT_BITS: u8 = 0b00000011;

    pub fn new(len_width_exponent: usize) -> Self {
        debug_assert!(len_width_exponent <= 3);

        Self {
            len_width_exponent: len_width_exponent.min(3),
        }
    }

    pub fn optimal(len: usize) -> Self {
        let len_width = match required_bytes_for_prim_int(len) {
            0 | 1 => 1,
            2 => 2,
            3..=4 => 4,
            5..=8 => 8,
            _ => unreachable!(),
        };
        let len_width_exponent = Self::len_width_exponent(len_width);
        Self { len_width_exponent }
    }

    pub fn exact(_len: usize) -> Self {
        let len_width = 8;
        let len_width_exponent = Self::len_width_exponent(len_width);
        Self { len_width_exponent }
    }

    pub fn len_width(&self) -> usize {
        1usize << self.len_width_exponent
    }

    fn len_width_exponent(len_width: usize) -> usize {
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
    fn decode(byte: u8) -> Result<Self, HeaderDecodeError> {
        HeaderType::Bytes.validate(byte)?;

        let byte = Byte(byte);

        let len_width_exponent = byte.masked_bits(Self::LEN_WIDTH_EXPONENT_BITS) as usize;

        Ok(Self { len_width_exponent })
    }
}

impl EncodeHeader for BytesHeader {
    fn encode(self) -> u8 {
        let mut byte = Byte(Self::TYPE_BITS);

        byte.set_bits_assert_masked_by(
            self.len_width_exponent as u8,
            Self::LEN_WIDTH_EXPONENT_BITS,
        );

        byte.0
    }
}

#[cfg(test)]
impl proptest::prelude::Arbitrary for BytesHeader {
    type Parameters = ();
    type Strategy = proptest::prelude::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy as _;
        (0..=8_usize).prop_map(Self::new).boxed()
    }
}
