#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

/// Header representing a unit value.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct UnitHeader;

impl UnitHeader {
    /// Creates a new header for a null value.
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl UnitHeader {
    pub(crate) const MASK: u8 = 0b00000001;
    pub(crate) const TYPE_BITS: u8 = 0b00000001;
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
        fn encode_decode_roundtrip(header in UnitHeader::arbitrary(), config in EncoderConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new_with_config(writer, config);
            encoder.encode_unit_header(&header).unwrap();

            prop_assert!(encoded.len() == 1);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::from_reader(reader);
            let decoded = decoder.decode_unit_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
