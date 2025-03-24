use std::{
    hash::{Hash, Hasher},
    num::TryFromIntError,
};

#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

use crate::num::{TryFromInt, TryIntoInt as _};

use super::SignedIntValue;

/// Represents an unsigned integer number.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Copy, Clone)]
pub enum UnsignedIntValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl Default for UnsignedIntValue {
    fn default() -> Self {
        Self::U8(0)
    }
}

macro_rules! impl_unsigned_int_value_from {
    ($t:ty => $v:ident) => {
        impl From<$t> for UnsignedIntValue {
            fn from(value: $t) -> Self {
                Self::$v(value)
            }
        }
    };
}

impl_unsigned_int_value_from!(u8 => U8);
impl_unsigned_int_value_from!(u16 => U16);
impl_unsigned_int_value_from!(u32 => U32);
impl_unsigned_int_value_from!(u64 => U64);

macro_rules! impl_try_from_unsigned_int_value {
    ($t:ty) => {
        impl TryFrom<UnsignedIntValue> for $t {
            type Error = std::num::TryFromIntError;

            fn try_from(value: UnsignedIntValue) -> Result<Self, Self::Error> {
                match value {
                    UnsignedIntValue::U8(value) => value.try_into_int(),
                    UnsignedIntValue::U16(value) => value.try_into_int(),
                    UnsignedIntValue::U32(value) => value.try_into_int(),
                    UnsignedIntValue::U64(value) => value.try_into_int(),
                }
            }
        }
    };
}

impl_try_from_unsigned_int_value!(u8);
impl_try_from_unsigned_int_value!(u16);
impl_try_from_unsigned_int_value!(u32);
impl_try_from_unsigned_int_value!(u64);
impl_try_from_unsigned_int_value!(usize);

impl PartialEq for UnsignedIntValue {
    fn eq(&self, other: &Self) -> bool {
        let lhs = match *self {
            Self::U8(value) => value as u64,
            Self::U16(value) => value as u64,
            Self::U32(value) => value as u64,
            Self::U64(value) => value,
        };
        let rhs = match *other {
            Self::U8(value) => value as u64,
            Self::U16(value) => value as u64,
            Self::U32(value) => value as u64,
            Self::U64(value) => value,
        };
        lhs == rhs
    }
}

impl PartialOrd for UnsignedIntValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for UnsignedIntValue {}

impl Ord for UnsignedIntValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.canonicalized().cmp(&other.canonicalized())
    }
}

impl Hash for UnsignedIntValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.canonicalized().hash(state)
    }
}

impl std::fmt::Debug for UnsignedIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self {
                Self::U8(value) => write!(f, "{value:#?}_u8"),
                Self::U16(value) => write!(f, "{value:#?}_u16"),
                Self::U32(value) => write!(f, "{value:#?}_u32"),
                Self::U64(value) => write!(f, "{value:#?}_u64"),
            }
        } else {
            match self {
                Self::U8(value) => std::fmt::Debug::fmt(value, f),
                Self::U16(value) => std::fmt::Debug::fmt(value, f),
                Self::U32(value) => std::fmt::Debug::fmt(value, f),
                Self::U64(value) => std::fmt::Debug::fmt(value, f),
            }
        }
    }
}

impl std::fmt::Display for UnsignedIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8(value) => std::fmt::Display::fmt(value, f),
            Self::U16(value) => std::fmt::Display::fmt(value, f),
            Self::U32(value) => std::fmt::Display::fmt(value, f),
            Self::U64(value) => std::fmt::Display::fmt(value, f),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for UnsignedIntValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::U8(value) => value.serialize(serializer),
            Self::U16(value) => value.serialize(serializer),
            Self::U32(value) => value.serialize(serializer),
            Self::U64(value) => value.serialize(serializer),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for UnsignedIntValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl serde::de::Visitor<'_> for ValueVisitor {
            type Value = UnsignedIntValue;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("unsigned integer value")
            }

            #[inline]
            fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(value.into())
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl UnsignedIntValue {
    pub fn to_signed(self) -> Result<SignedIntValue, TryFromIntError> {
        match self {
            Self::U8(unsigned) => {
                if unsigned <= i8::MAX as u8 {
                    i8::try_from_int(unsigned).map(SignedIntValue::I8)
                } else {
                    i16::try_from_int(unsigned).map(SignedIntValue::I16)
                }
            }
            Self::U16(unsigned) => {
                if unsigned <= i16::MAX as u16 {
                    i16::try_from_int(unsigned).map(SignedIntValue::I16)
                } else {
                    i32::try_from_int(unsigned).map(SignedIntValue::I32)
                }
            }
            Self::U32(unsigned) => {
                if unsigned <= i32::MAX as u32 {
                    i32::try_from_int(unsigned).map(SignedIntValue::I32)
                } else {
                    i64::try_from_int(unsigned).map(SignedIntValue::I64)
                }
            }
            Self::U64(unsigned) => i64::try_from_int(unsigned).map(SignedIntValue::I64),
        }
    }

    pub(crate) fn canonicalized(&self) -> u64 {
        match *self {
            Self::U8(value) => value as u64,
            Self::U16(value) => value as u64,
            Self::U32(value) => value as u64,
            Self::U64(value) => value,
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
        fn eq(lhs in u8::MIN..=u8::MAX, rhs in u8::MIN..=u8::MAX) {

            let lhs_values = [
                UnsignedIntValue::U8(lhs),
                UnsignedIntValue::U16(lhs as u16),
                UnsignedIntValue::U32(lhs as u32),
                UnsignedIntValue::U64(lhs as u64),
            ];

            let rhs_values = [
                UnsignedIntValue::U8(rhs),
                UnsignedIntValue::U16(rhs as u16),
                UnsignedIntValue::U32(rhs as u32),
                UnsignedIntValue::U64(rhs as u64),
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
        fn ord(lhs in u8::MIN..=u8::MAX, rhs in u8::MIN..=u8::MAX) {
            let lhs_values = [
                UnsignedIntValue::U8(lhs),
                UnsignedIntValue::U16(lhs as u16),
                UnsignedIntValue::U32(lhs as u32),
                UnsignedIntValue::U64(lhs as u64),
            ];

            let rhs_values = [
                UnsignedIntValue::U8(rhs),
                UnsignedIntValue::U16(rhs as u16),
                UnsignedIntValue::U32(rhs as u32),
                UnsignedIntValue::U64(rhs as u64),
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
        fn hash(lhs in u8::MIN..=u8::MAX) {
            use std::hash::BuildHasher as _;

            let values = [
                UnsignedIntValue::U8(lhs),
                UnsignedIntValue::U16(lhs as u16),
                UnsignedIntValue::U32(lhs as u32),
                UnsignedIntValue::U64(lhs as u64),
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
        assert_eq!(format!("{}", UnsignedIntValue::from(42_u8)), "42");
        assert_eq!(format!("{}", UnsignedIntValue::from(42_u16)), "42");
        assert_eq!(format!("{}", UnsignedIntValue::from(42_u32)), "42");
        assert_eq!(format!("{}", UnsignedIntValue::from(42_u64)), "42");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", UnsignedIntValue::from(42_u8)), "42");
        assert_eq!(format!("{:?}", UnsignedIntValue::from(42_u16)), "42");
        assert_eq!(format!("{:?}", UnsignedIntValue::from(42_u32)), "42");
        assert_eq!(format!("{:?}", UnsignedIntValue::from(42_u64)), "42");

        assert_eq!(format!("{:#?}", UnsignedIntValue::from(42_u8)), "42_u8");
        assert_eq!(format!("{:#?}", UnsignedIntValue::from(42_u16)), "42_u16");
        assert_eq!(format!("{:#?}", UnsignedIntValue::from(42_u32)), "42_u32");
        assert_eq!(format!("{:#?}", UnsignedIntValue::from(42_u64)), "42_u64");
    }

    proptest! {
        #[test]
        fn encode_decode_roundtrip(value in UnsignedIntValue::arbitrary(), config in EncoderConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_unsigned_int_value(&value).unwrap();

            prop_assert!(encoded.len() <= 1 + 8);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_unsigned_int_value().unwrap();
            prop_assert_eq!(&decoded, &value);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_value().unwrap();
            let Value::Int(decoded) = decoded else {
                panic!("expected int value");
            };
            prop_assert_eq!(&decoded, &IntValue::Unsigned(value));
        }
    }
}
