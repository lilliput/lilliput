#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

/// Represents a boolean.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BoolValue(pub bool);

impl From<bool> for BoolValue {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<BoolValue> for bool {
    fn from(value: BoolValue) -> Self {
        value.0
    }
}

impl std::fmt::Debug for BoolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::fmt::Display for BoolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
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
        value::Value,
    };

    use super::*;

    #[test]
    fn display() {
        assert_eq!(format!("{}", BoolValue::from(false)), "false");
        assert_eq!(format!("{}", BoolValue::from(true)), "true");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", BoolValue::from(false)), "false");
        assert_eq!(format!("{:?}", BoolValue::from(true)), "true");

        assert_eq!(format!("{:#?}", BoolValue::from(false)), "false");
        assert_eq!(format!("{:#?}", BoolValue::from(true)), "true");
    }

    proptest! {
        #[test]
        fn encode_decode_roundtrip(value in BoolValue::arbitrary(), config in EncoderConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_bool(value.0).unwrap();
            prop_assert_eq!(encoded.len(), 1);

            prop_assert!(encoded.len() == 1);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_bool().unwrap();
            prop_assert_eq!(decoded, value.0);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_value().unwrap();
            let Value::Bool(decoded) = decoded else {
                panic!("expected bool value");
            };
            prop_assert_eq!(&decoded, &value);
        }
    }
}
