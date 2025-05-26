#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

/// Represents a floating-point number.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FloatHeader {
    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(strategy = "(1..=FloatHeader::MAX_VALUE_WIDTH)")
    )]
    width: u8,
}

impl FloatHeader {
    pub fn new(width: u8) -> Self {
        assert!(width >= 1);
        assert!(width <= 8);

        Self { width }
    }

    pub fn width(&self) -> u8 {
        self.width
    }
}

impl FloatHeader {
    pub const MASK: u8 = 0b00001111;
    pub const MAX_VALUE_WIDTH: u8 = Self::VALUE_WIDTH_BITS + 1;

    pub(crate) const TYPE_BITS: u8 = 0b00001000;

    pub(crate) const VALUE_WIDTH_BITS: u8 = 0b00000111;
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

    proptest::proptest! {
        #[test]
        fn encode_decode_roundtrip(header in FloatHeader::arbitrary(), config in EncoderConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_float_header(&header).unwrap();

            prop_assert!(encoded.len() == 1);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_float_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
