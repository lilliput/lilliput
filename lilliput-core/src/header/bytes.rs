#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

/// Header representing a byte sequence.
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
    /// Creates a header from a byte array's length.
    #[inline]
    pub fn for_len(len: usize) -> Self {
        Self { len }
    }

    /// Returns `true` if the associated value has a length of zero bytes, otherwise `false`.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the associated value's length.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
}

impl BytesHeader {
    pub(crate) const MASK: u8 = 0b00000111;
    pub(crate) const TYPE_BITS: u8 = 0b00000100;

    pub(crate) const LEN_WIDTH_EXPONENT_BITS: u8 = 0b00000011;
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
        fn encode_decode_roundtrip(header in BytesHeader::arbitrary(), config in EncoderConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new_with_config(writer, config);
            encoder.encode_bytes_header(&header).unwrap();

            prop_assert!(encoded.len() <= 1 + 8);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_bytes_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
