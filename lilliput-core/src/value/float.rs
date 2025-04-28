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
        config::EncodingConfig,
        decoder::Decoder,
        encoder::Encoder,
        io::{SliceReader, VecWriter},
        value::Value,
    };

    use super::*;

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
        fn encode_decode_roundtrip(value in FloatValue::arbitrary(), config in EncodingConfig::arbitrary()) {
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
    }
}
