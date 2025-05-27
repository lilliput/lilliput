use std::{
    hash::{Hash, Hasher},
    num::TryFromIntError,
};

#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

use crate::num::{TryFromInt, TryIntoInt as _};

use super::UnsignedIntValue;

/// Represents a signed integer number.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Copy, Clone)]
pub enum SignedIntValue {
    /// 8-bit value.
    I8(i8),
    /// 16-bit value.
    I16(i16),
    /// 32-bit value.
    I32(i32),
    /// 64-bit value.
    I64(i64),
}

impl Default for SignedIntValue {
    fn default() -> Self {
        Self::I8(0)
    }
}

macro_rules! impl_signed_int_value_from {
    ($t:ty => $v:ident) => {
        impl From<$t> for SignedIntValue {
            fn from(value: $t) -> Self {
                Self::$v(value)
            }
        }
    };
}

impl_signed_int_value_from!(i8 => I8);
impl_signed_int_value_from!(i16 => I16);
impl_signed_int_value_from!(i32 => I32);
impl_signed_int_value_from!(i64 => I64);

macro_rules! impl_try_from_signed_int_value {
    ($t:ty) => {
        impl TryFrom<SignedIntValue> for $t {
            type Error = std::num::TryFromIntError;

            fn try_from(value: SignedIntValue) -> Result<Self, Self::Error> {
                match value {
                    SignedIntValue::I8(value) => value.try_into_int(),
                    SignedIntValue::I16(value) => value.try_into_int(),
                    SignedIntValue::I32(value) => value.try_into_int(),
                    SignedIntValue::I64(value) => value.try_into_int(),
                }
            }
        }
    };
}

impl_try_from_signed_int_value!(i8);
impl_try_from_signed_int_value!(i16);
impl_try_from_signed_int_value!(i32);
impl_try_from_signed_int_value!(i64);
impl_try_from_signed_int_value!(isize);

impl PartialEq for SignedIntValue {
    fn eq(&self, other: &Self) -> bool {
        let lhs = match *self {
            Self::I8(value) => value as i64,
            Self::I16(value) => value as i64,
            Self::I32(value) => value as i64,
            Self::I64(value) => value,
        };
        let rhs = match *other {
            Self::I8(value) => value as i64,
            Self::I16(value) => value as i64,
            Self::I32(value) => value as i64,
            Self::I64(value) => value,
        };
        lhs == rhs
    }
}

impl PartialOrd for SignedIntValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SignedIntValue {}

impl Ord for SignedIntValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.canonicalized().cmp(&other.canonicalized())
    }
}

impl Hash for SignedIntValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.canonicalized().hash(state)
    }
}

impl std::fmt::Debug for SignedIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self {
                Self::I8(value) => write!(f, "{value:#?}_i8"),
                Self::I16(value) => write!(f, "{value:#?}_i16"),
                Self::I32(value) => write!(f, "{value:#?}_i32"),
                Self::I64(value) => write!(f, "{value:#?}_i64"),
            }
        } else {
            match self {
                Self::I8(value) => std::fmt::Debug::fmt(value, f),
                Self::I16(value) => std::fmt::Debug::fmt(value, f),
                Self::I32(value) => std::fmt::Debug::fmt(value, f),
                Self::I64(value) => std::fmt::Debug::fmt(value, f),
            }
        }
    }
}

impl std::fmt::Display for SignedIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8(value) => std::fmt::Display::fmt(value, f),
            Self::I16(value) => std::fmt::Display::fmt(value, f),
            Self::I32(value) => std::fmt::Display::fmt(value, f),
            Self::I64(value) => std::fmt::Display::fmt(value, f),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for SignedIntValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::I8(value) => value.serialize(serializer),
            Self::I16(value) => value.serialize(serializer),
            Self::I32(value) => value.serialize(serializer),
            Self::I64(value) => value.serialize(serializer),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SignedIntValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl serde::de::Visitor<'_> for ValueVisitor {
            type Value = SignedIntValue;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("signed integer value")
            }

            #[inline]
            fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(value.into())
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl SignedIntValue {
    /// Attempts to convert the value into an unsigned value.
    pub fn to_unsigned(self) -> Result<UnsignedIntValue, TryFromIntError> {
        match self {
            Self::I8(signed) => u8::try_from_int(signed).map(UnsignedIntValue::U8),
            Self::I16(signed) => u16::try_from_int(signed).map(UnsignedIntValue::U16),
            Self::I32(signed) => u32::try_from_int(signed).map(UnsignedIntValue::U32),
            Self::I64(signed) => u64::try_from_int(signed).map(UnsignedIntValue::U64),
        }
    }

    pub(crate) fn canonicalized(&self) -> i64 {
        match *self {
            Self::I8(value) => value as i64,
            Self::I16(value) => value as i64,
            Self::I32(value) => value as i64,
            Self::I64(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::hash::RandomState;

    use proptest::prelude::*;
    use test_log::test;

    use crate::{
        config::EncoderConfig,
        decoder::Decoder,
        encoder::Encoder,
        io::{SliceReader, VecWriter},
        value::{IntValue, Value},
    };

    use super::*;

    proptest! {
        #[test]
        fn eq(lhs in i8::MIN..=i8::MAX, rhs in i8::MIN..=i8::MAX) {

            let lhs_values = [
                SignedIntValue::I8(lhs),
                SignedIntValue::I16(lhs as i16),
                SignedIntValue::I32(lhs as i32),
                SignedIntValue::I64(lhs as i64),
            ];

            let rhs_values = [
                SignedIntValue::I8(rhs),
                SignedIntValue::I16(rhs as i16),
                SignedIntValue::I32(rhs as i32),
                SignedIntValue::I64(rhs as i64),
            ];

            for lhs_value in &lhs_values {
                for rhs_value in &rhs_values {
                    let int_is_equal = lhs.eq(&rhs);
                    prop_assert_eq!(lhs_value.eq(rhs_value), int_is_equal);
                    prop_assert_eq!(rhs_value.eq(lhs_value), int_is_equal);
                }
            }
        }

        #[test]
        fn ord(lhs in i8::MIN..=i8::MAX, rhs in i8::MIN..=i8::MAX) {
            let lhs_values = [
                SignedIntValue::I8(lhs),
                SignedIntValue::I16(lhs as i16),
                SignedIntValue::I32(lhs as i32),
                SignedIntValue::I64(lhs as i64),
            ];

            let rhs_values = [
                SignedIntValue::I8(rhs),
                SignedIntValue::I16(rhs as i16),
                SignedIntValue::I32(rhs as i32),
                SignedIntValue::I64(rhs as i64),
            ];

            for lhs_value in &lhs_values {
                for rhs_value in &rhs_values {
                    let int_ordering = lhs.cmp(&rhs);
                    prop_assert_eq!(lhs_value.cmp(rhs_value), int_ordering);
                    prop_assert_eq!(rhs_value.cmp(lhs_value), int_ordering.reverse());
                }
            }
        }

        #[test]
        fn hash(lhs in i8::MIN..=i8::MAX) {
            use std::hash::BuildHasher as _;

            let values = [
                SignedIntValue::I8(lhs),
                SignedIntValue::I16(lhs as i16),
                SignedIntValue::I32(lhs as i32),
                SignedIntValue::I64(lhs as i64),
            ];

            for lhs_value in &values {
                for rhs_value in &values {
                    let build_hasher = RandomState::new();
                    let lhs_hash = build_hasher.hash_one(lhs_value);
                    let rhs_hash = build_hasher.hash_one(rhs_value);
                    prop_assert_eq!(lhs_hash, rhs_hash);
                }
            }
        }
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SignedIntValue::from(42_i8)), "42");
        assert_eq!(format!("{}", SignedIntValue::from(42_i16)), "42");
        assert_eq!(format!("{}", SignedIntValue::from(42_i32)), "42");
        assert_eq!(format!("{}", SignedIntValue::from(42_i64)), "42");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SignedIntValue::from(42_i8)), "42");
        assert_eq!(format!("{:?}", SignedIntValue::from(42_i16)), "42");
        assert_eq!(format!("{:?}", SignedIntValue::from(42_i32)), "42");
        assert_eq!(format!("{:?}", SignedIntValue::from(42_i64)), "42");

        assert_eq!(format!("{:#?}", SignedIntValue::from(42_i8)), "42_i8");
        assert_eq!(format!("{:#?}", SignedIntValue::from(42_i16)), "42_i16");
        assert_eq!(format!("{:#?}", SignedIntValue::from(42_i32)), "42_i32");
        assert_eq!(format!("{:#?}", SignedIntValue::from(42_i64)), "42_i64");
    }

    proptest! {
        #[test]
        fn encode_decode_roundtrip(value in SignedIntValue::arbitrary(), config in EncoderConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new_with_config(writer, config);
            encoder.encode_signed_int_value(&value).unwrap();

            prop_assert!(encoded.len() <= 1 + 8);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_signed_int_value().unwrap();
            prop_assert_eq!(&decoded, &value);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_value().unwrap();
            let Value::Int(decoded) = decoded else {
                panic!("expected int value");
            };
            prop_assert_eq!(&decoded, &IntValue::Signed(value));
        }
    }
}
