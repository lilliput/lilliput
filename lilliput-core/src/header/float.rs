#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

use crate::{config::PackingMode, num::WithPackedBeBytes};

/// Represents a floating-point number.
///
/// # Binary representation
///
/// ```plain
/// 0b00001XXX <FLOAT>
///   ├───┘├─┘  └─ Value
///   │    └─ Width in bytes, minus 1
///   └─ Float Type
/// ```
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

    pub fn for_f32(value: f32, packing_mode: PackingMode) -> Self {
        value.with_packed_be_bytes(packing_mode, Self::for_float_be_bytes)
    }

    pub fn for_f64(value: f64, packing_mode: PackingMode) -> Self {
        value.with_packed_be_bytes(packing_mode, Self::for_float_be_bytes)
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    fn for_float_be_bytes(be_bytes: &[u8]) -> FloatHeader {
        let width = be_bytes.len();
        assert!((1..=8).contains(&width));
        FloatHeader::new(width as u8)
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
        config::EncodingConfig,
        decoder::Decoder,
        encoder::Encoder,
        io::{SliceReader, VecWriter},
    };

    use super::*;

    proptest::proptest! {
        #[test]
        fn for_f32(value in f32::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = FloatHeader::for_f32(value, packing_mode);
            let width = header.width();

            match packing_mode {
                PackingMode::None => prop_assert!(width == 4),
                PackingMode::Native => prop_assert!([4].contains(&width)),
                PackingMode::Optimal => prop_assert!((1..=4).contains(&width)),
            }
        }

        #[test]
        fn for_f64(value in f64::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = FloatHeader::for_f64(value, packing_mode);
            let width = header.width();

            match packing_mode {
                PackingMode::None => prop_assert!(width == 8),
                PackingMode::Native => prop_assert!([4, 8].contains(&width)),
                PackingMode::Optimal => prop_assert!((1..=8).contains(&width)),
            }
        }

        #[test]
        fn encode_decode_roundtrip(header in FloatHeader::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_float_header(&header).unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_float_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
