#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

/// Represents a null value.
///
/// # Binary representation
///
/// ```plain
/// 0b00000000
///   ├──────┘
///   └─ Null Type
/// ```
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
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

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use test_log::test;

    use crate::{
        config::EncoderConfig,
        decoder::Decoder,
        encoder::Encoder,
        io::{SliceReader, VecWriter},
    };

    use super::*;

    proptest! {
        #[test]
        fn encode_decode_roundtrip(header in NullHeader::arbitrary(), config in EncoderConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_null_header(&header).unwrap();

            prop_assert!(encoded.len() == 1);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_null_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
