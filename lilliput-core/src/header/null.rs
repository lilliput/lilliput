use crate::binary::Byte;

use super::{DecodeHeader, EncodeHeader, Expectation, Marker};

/// Represents a null value.
///
/// # Binary representation
///
/// ```plain
/// 0b00000001
///   ├──────┘
///   └─ Null Type
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NullHeader;

impl NullHeader {
    const TYPE_BITS: u8 = 0b00000001;
}

impl DecodeHeader for NullHeader {
    fn decode(byte: u8) -> Result<Self, Expectation<Marker>> {
        Marker::Null.validate(byte)?;

        Ok(Self)
    }
}

impl EncodeHeader for NullHeader {
    fn encode(self) -> u8 {
        let byte = Byte(Self::TYPE_BITS);

        byte.0
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::prelude::Arbitrary for NullHeader {
    type Parameters = ();
    type Strategy = proptest::prelude::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        Just(NullHeader).boxed()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn encode_decode_roundtrip(header in NullHeader::arbitrary()) {
            let encoded = header.encode();
            let decoded = NullHeader::decode(encoded).unwrap();

            prop_assert_eq!(&decoded, &header);
        }
    }
}
