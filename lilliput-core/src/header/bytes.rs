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
    len: usize,
}

impl BytesHeader {
    #[inline]
    pub fn new(len: usize) -> Self {
        Self { len }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
}

impl BytesHeader {
    pub const MASK: u8 = 0b00000111;
    pub(crate) const TYPE_BITS: u8 = 0b00000100;

    pub(crate) const LEN_WIDTH_EXPONENT_BITS: u8 = 0b00000011;
}

#[cfg(any(test, feature = "testing"))]
impl proptest::prelude::Arbitrary for BytesHeader {
    type Parameters = ();
    type Strategy = proptest::prelude::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy as _;
        proptest::prop_oneof![
            proptest::num::u8::ANY.prop_map(|n| n as usize),
            proptest::num::u16::ANY.prop_map(|n| n as usize),
            proptest::num::u32::ANY.prop_map(|n| n as usize),
            proptest::num::u64::ANY.prop_map(|n| n as usize),
        ]
        .prop_map(Self::new)
        .boxed()
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
        fn encode_decode_roundtrip(header in BytesHeader::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_bytes_header(&header).unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_bytes_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
