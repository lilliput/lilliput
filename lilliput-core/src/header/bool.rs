use crate::binary::Byte;

use super::{DecodeHeader, EncodeHeader, HeaderDecodeError, HeaderType};

/// Represents a boolean.
///
/// # Binary representation
///
/// ```plain
/// 0b0000001X
///   ├─────┘└─ Value (0 = false, 1 = true)
///   └─ Data Type
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BoolHeader {
    pub value: bool,
}

impl BoolHeader {
    const TYPE_BITS: u8 = 0b0000010;
    const VALUE_BIT: u8 = 0b0000001;

    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn value(&self) -> bool {
        self.value
    }
}

impl DecodeHeader for BoolHeader {
    fn decode(byte: u8) -> Result<Self, HeaderDecodeError> {
        HeaderType::Bool.validate(byte)?;

        let byte = Byte(byte);

        let value = byte.contains_bits(Self::VALUE_BIT);

        Ok(Self { value })
    }
}

impl EncodeHeader for BoolHeader {
    fn encode(self) -> u8 {
        let mut byte = Byte(Self::TYPE_BITS);

        byte.set_bits_if(Self::VALUE_BIT, self.value);

        byte.0
    }
}

#[cfg(test)]
impl proptest::prelude::Arbitrary for BoolHeader {
    type Parameters = ();
    type Strategy = proptest::prelude::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy as _;
        proptest::bool::ANY.prop_map(Self::new).boxed()
    }
}
