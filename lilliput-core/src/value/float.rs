use std::hash::{Hash, Hasher};

#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

use decorum::{constraint::IsFloat, proxy::Constrained};

/// Represents a floating-point number.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Copy, Clone)]
pub enum FloatValue {
    F32(f32),
    F64(f64),
}

impl FloatValue {
    pub fn as_f32(self) -> f32 {
        match self {
            FloatValue::F32(value) => value,
            FloatValue::F64(value) => value as f32,
        }
    }

    pub fn as_f64(self) -> f64 {
        match self {
            FloatValue::F32(value) => value as f64,
            FloatValue::F64(value) => value,
        }
    }
}

impl Default for FloatValue {
    fn default() -> Self {
        Self::F32(0.0)
    }
}

impl From<f32> for FloatValue {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<f64> for FloatValue {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}

impl From<FloatValue> for f32 {
    fn from(value: FloatValue) -> Self {
        value.as_f32()
    }
}

impl From<FloatValue> for f64 {
    fn from(value: FloatValue) -> Self {
        value.as_f64()
    }
}

impl Eq for FloatValue {}

impl PartialEq for FloatValue {
    fn eq(&self, other: &Self) -> bool {
        self.canonical_total().eq(&other.canonical_total())
    }
}

impl Ord for FloatValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.canonical_total().cmp(&other.canonical_total())
    }
}

impl PartialOrd for FloatValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for FloatValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.canonical_total().hash(state);
    }
}

impl std::fmt::Debug for FloatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self {
                Self::F32(value) => write!(f, "{value:#?}_f32"),
                Self::F64(value) => write!(f, "{value:#?}_f64"),
            }
        } else {
            match self {
                Self::F32(value) => std::fmt::Debug::fmt(value, f),
                Self::F64(value) => std::fmt::Debug::fmt(value, f),
            }
        }
    }
}

impl std::fmt::Display for FloatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::F32(value) => std::fmt::Display::fmt(value, f),
            Self::F64(value) => std::fmt::Display::fmt(value, f),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for FloatValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::F32(value) => value.serialize(serializer),
            Self::F64(value) => value.serialize(serializer),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for FloatValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl serde::de::Visitor<'_> for ValueVisitor {
            type Value = FloatValue;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("floating-point value")
            }

            #[inline]
            fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
                Ok(value.into())
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl FloatValue {
    fn canonical_total(self) -> Constrained<f64, IsFloat> {
        decorum::Total::assert(self.as_f64())
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use test_log::test;

    use crate::{
        config::{EncoderConfig, PackingMode},
        decoder::Decoder,
        encoder::Encoder,
        io::{SliceReader, VecWriter},
        value::Value,
    };

    use super::*;

    fn non_normal_or_subnormal_f32() -> impl Strategy<Value = f32> {
        proptest::prop_oneof![
            proptest::num::f32::SIGNALING_NAN,
            proptest::num::f32::QUIET_NAN,
            proptest::num::f32::INFINITE,
            proptest::num::f32::ZERO,
        ]
    }

    fn non_normal_or_subnormal_f64() -> impl Strategy<Value = f64> {
        proptest::prop_oneof![
            proptest::num::f64::SIGNALING_NAN,
            proptest::num::f64::QUIET_NAN,
            proptest::num::f64::INFINITE,
            proptest::num::f64::ZERO,
        ]
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", FloatValue::from(4.2_f32)), "4.2");
        assert_eq!(format!("{}", FloatValue::from(4.2_f64)), "4.2");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", FloatValue::from(4.2_f32)), "4.2");
        assert_eq!(format!("{:?}", FloatValue::from(4.2_f64)), "4.2");

        assert_eq!(format!("{:#?}", FloatValue::from(4.2_f32)), "4.2_f32");
        assert_eq!(format!("{:#?}", FloatValue::from(4.2_f64)), "4.2_f64");
    }

    proptest! {
        #[test]
        fn encode_decode_roundtrip(value in FloatValue::arbitrary(), config in EncoderConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_float_value(&value).unwrap();

            prop_assert!(encoded.len() <= 1 + 8);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_float_value().unwrap();
            prop_assert_eq!(&decoded, &value);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_value().unwrap();
            let Value::Float(decoded) = decoded else {
                panic!("expected float value");
            };
            prop_assert_eq!(&decoded, &value);
        }

        #[test]
        fn non_normal_or_subnormal_f32_encodes_optimally(value in non_normal_or_subnormal_f32()) {
            let config = EncoderConfig::default().with_packing(PackingMode::Optimal);

            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_f32(value).unwrap();

            prop_assert!(encoded.len() == 2, "value should optimally pack to single byte");
        }

        #[test]
        fn non_normal_or_subnormal_f64_encodes_optimally(value in non_normal_or_subnormal_f64()) {
            let config = EncoderConfig::default().with_packing(PackingMode::Optimal);

            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_f64(value).unwrap();

            prop_assert!(encoded.len() == 2, "value should optimally pack to single byte");
        }
    }
}
