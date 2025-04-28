#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

/// Represents a null value.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NullValue;

impl From<()> for NullValue {
    fn from(_value: ()) -> Self {
        Self
    }
}

impl std::fmt::Debug for NullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

impl std::fmt::Display for NullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
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
        value::Value,
    };

    use super::*;

    #[test]
    fn display() {
        assert_eq!(format!("{NullValue}"), "null");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{NullValue:?}"), "null");
        assert_eq!(format!("{NullValue:#?}"), "null");
    }

    proptest! {
        #[test]
        fn encode_decode_roundtrip(value in NullValue::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_null().unwrap();

            prop_assert!(encoded.len() == 1);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            decoder.decode_null().unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_value().unwrap();
            let Value::Null(decoded) = decoded else {
                panic!("expected null value");
            };
            prop_assert_eq!(&decoded, &value);
        }
    }
}
