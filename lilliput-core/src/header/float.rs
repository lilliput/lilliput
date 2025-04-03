use crate::value::FloatValue;

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
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FloatHeader {
    value: FloatValue,
}

impl FloatHeader {
    #[inline]
    pub(crate) fn new(value: FloatValue) -> Self {
        Self { value }
    }

    #[inline]
    pub fn value(&self) -> FloatValue {
        self.value
    }
}

impl FloatHeader {
    pub(crate) const TYPE_BITS: u8 = 0b00001000;

    pub(crate) const VALUE_WIDTH_BITS: u8 = 0b00000111;
}

#[cfg(any(test, feature = "testing"))]
impl proptest::prelude::Arbitrary for FloatHeader {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;
        proptest::prop_oneof![
            proptest::num::f32::ANY.prop_map(|f| Self::new(f.into())),
            proptest::num::f64::ANY.prop_map(|f| Self::new(f.into())),
        ]
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
