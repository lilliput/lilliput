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
    value: bool,
}

impl BoolHeader {
    #[inline]
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    #[inline]
    pub fn value(&self) -> bool {
        self.value
    }
}

impl BoolHeader {
    pub(crate) const TYPE_BITS: u8 = 0b0000010;
    pub(crate) const VALUE_BIT: u8 = 0b0000001;
}

#[cfg(any(test, feature = "testing"))]
impl proptest::prelude::Arbitrary for BoolHeader {
    type Parameters = ();
    type Strategy = proptest::prelude::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy as _;
        proptest::bool::ANY.prop_map(Self::new).boxed()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::{
        config::EncodingConfig,
        decoder::Decoder,
        encoder::Encoder,
        io::{SliceReader, VecWriter},
    };

    use super::*;

    proptest! {
        #[test]
        fn encode_decode_roundtrip(header in BoolHeader::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_bool_header(&header).unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_bool_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
