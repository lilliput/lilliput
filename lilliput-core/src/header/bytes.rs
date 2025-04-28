#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

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
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BytesHeader {
    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(strategy = "super::arbitrary_len()")
    )]
    len: usize,
}

impl BytesHeader {
    #[inline]
    pub fn for_len(len: usize) -> Self {
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
        fn encode_decode_roundtrip(header in BytesHeader::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_bytes_header(&header).unwrap();

            prop_assert!(encoded.len() <= 1 + 8);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_bytes_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
