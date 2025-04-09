/// Represents a null value.
///
/// # Binary representation
///
/// ```plain
/// 0b00000000
///   ├──────┘
///   └─ Null Type
/// ```
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct NullHeader;

impl NullHeader {
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl NullHeader {
    pub const MASK: u8 = 0b00000000;
    pub(crate) const TYPE_BITS: u8 = 0b00000000;
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
    use test_log::test;

    use crate::{
        config::EncodingConfig,
        decoder::Decoder,
        encoder::Encoder,
        io::{SliceReader, VecWriter},
    };

    use super::*;

    proptest! {
        #[test]
        fn encode_decode_roundtrip(header in NullHeader::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_null_header(&header).unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_null_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
