use crate::binary::Byte;

use super::{DecodeHeader, EncodeHeader, Expectation, Marker};

/// Represents a floating-point number.
///
/// # Binary representation
///
/// ```plain
/// 0b00001XXX <FLOAT>
///   ├───┘├─┘  └─ Value
///   │    └─ Width in bytes, minus 1
///   └─ Float Type
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FloatHeader {
    width: u8,
}

impl FloatHeader {
    const TYPE_BITS: u8 = 0b00001000;

    const VALUE_WIDTH_BITS: u8 = 0b00000111;

    #[inline]
    pub fn new(width: usize) -> Self {
        debug_assert!(width <= 8);

        Self { width: width as u8 }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width.into()
    }
}

impl DecodeHeader for FloatHeader {
    fn decode(byte: u8) -> Result<Self, Expectation<Marker>> {
        Marker::Float.validate(byte)?;

        let byte = Byte(byte);

        let width = byte.masked_bits(Self::VALUE_WIDTH_BITS) + 1;

        Ok(Self { width })
    }
}

impl EncodeHeader for FloatHeader {
    fn encode(self) -> u8 {
        let mut byte = Byte(Self::TYPE_BITS);

        debug_assert!(self.width <= 8);
        byte.set_bits(self.width - 1);

        byte.0
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::prelude::Arbitrary for FloatHeader {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;

        (1..=8_usize).prop_map(Self::new).boxed()
    }
}
