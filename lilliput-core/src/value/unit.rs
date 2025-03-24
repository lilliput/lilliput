#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

/// Represents a unit value.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UnitValue;

impl From<()> for UnitValue {
    fn from(_: ()) -> Self {
        Self
    }
}

impl std::fmt::Debug for UnitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unit")
    }
}

impl std::fmt::Display for UnitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unit")
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for UnitValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_unit()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for UnitValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct UnitValueVisitor;

        impl<'de> serde::de::Visitor<'de> for UnitValueVisitor {
            type Value = UnitValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("unit value")
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(UnitValue)
            }
        }

        deserializer.deserialize_unit(UnitValueVisitor)
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
        assert_eq!(format!("{UnitValue}"), "unit");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{UnitValue:?}"), "unit");
        assert_eq!(format!("{UnitValue:#?}"), "unit");
    }

    proptest! {
        #[test]
        fn encode_decode_roundtrip(value in UnitValue::arbitrary(), config in EncoderConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_unit().unwrap();

            prop_assert!(encoded.len() <= 1);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            decoder.decode_unit().unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_value().unwrap();
            let Value::Unit(decoded) = decoded else {
                panic!("expected unit value");
            };
            prop_assert_eq!(&decoded, &value);
        }
    }
}
