//! Values.

#[cfg(any(test, feature = "testing"))]
use proptest::{prelude::*, sample::SizeRange};

mod bool;
mod bytes;
mod float;
mod int;
mod map;
mod null;
mod seq;
mod string;
mod unit;

pub use self::{
    bool::BoolValue,
    bytes::BytesValue,
    float::FloatValue,
    int::{IntValue, SignedIntValue, UnsignedIntValue},
    map::{Map, MapValue},
    null::NullValue,
    seq::{Seq, SeqValue},
    string::StringValue,
    unit::UnitValue,
};

/// Represents a value.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Value {
    /// Represents a integer number.
    Int(IntValue),

    /// Represents a string.
    String(StringValue),

    /// Represents a sequence of values.
    Seq(SeqValue),

    /// Represents a map of key-value pairs.
    ///
    /// By default the map is backed by a `BTreeMap`. Enable the `preserve_order`
    /// feature of serde_lilliput to use `OrderMap` instead, which preserves
    /// entries in the order they are inserted into the map.
    Map(MapValue),

    /// Represents a floating-point number.
    Float(FloatValue),

    /// Represents a byte array.
    Bytes(BytesValue),

    /// Represents a boolean.
    Bool(BoolValue),

    /// Represents a unit value.
    Unit(UnitValue),

    /// Represents a null value.
    Null(NullValue),
}

impl Default for Value {
    fn default() -> Self {
        Self::Null(NullValue)
    }
}

impl From<IntValue> for Value {
    fn from(value: IntValue) -> Self {
        Self::Int(value)
    }
}

impl From<StringValue> for Value {
    fn from(value: StringValue) -> Self {
        Self::String(value)
    }
}

impl From<SeqValue> for Value {
    fn from(value: SeqValue) -> Self {
        Self::Seq(value)
    }
}

impl From<MapValue> for Value {
    fn from(value: MapValue) -> Self {
        Self::Map(value)
    }
}

impl From<FloatValue> for Value {
    fn from(value: FloatValue) -> Self {
        Self::Float(value)
    }
}

impl From<BytesValue> for Value {
    fn from(value: BytesValue) -> Self {
        Self::Bytes(value)
    }
}

impl From<BoolValue> for Value {
    fn from(value: BoolValue) -> Self {
        Self::Bool(value)
    }
}

impl From<UnitValue> for Value {
    fn from(value: UnitValue) -> Self {
        Self::Unit(value)
    }
}

impl From<NullValue> for Value {
    fn from(value: NullValue) -> Self {
        Self::Null(value)
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self {
                Self::Int(value) => f.debug_tuple("Int").field(value).finish(),
                Self::String(value) => f.debug_tuple("String").field(value).finish(),
                Self::Seq(value) => f.debug_tuple("Seq").field(value).finish(),
                Self::Map(value) => f.debug_tuple("Map").field(value).finish(),
                Self::Float(value) => f.debug_tuple("Float").field(value).finish(),
                Self::Bytes(value) => f.debug_tuple("Bytes").field(value).finish(),
                Self::Bool(value) => f.debug_tuple("Bool").field(value).finish(),
                Self::Unit(value) => f.debug_tuple("Unit").field(value).finish(),
                Self::Null(value) => f.debug_tuple("Null").field(value).finish(),
            }
        } else {
            match self {
                Self::Int(value) => std::fmt::Debug::fmt(value, f),
                Self::String(value) => std::fmt::Debug::fmt(value, f),
                Self::Seq(value) => std::fmt::Debug::fmt(value, f),
                Self::Map(value) => std::fmt::Debug::fmt(value, f),
                Self::Float(value) => std::fmt::Debug::fmt(value, f),
                Self::Bytes(value) => std::fmt::Debug::fmt(value, f),
                Self::Bool(value) => std::fmt::Debug::fmt(value, f),
                Self::Unit(value) => std::fmt::Debug::fmt(value, f),
                Self::Null(value) => std::fmt::Debug::fmt(value, f),
            }
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Int(value) => value.serialize(serializer),
            Value::String(value) => value.serialize(serializer),
            Value::Seq(value) => value.serialize(serializer),
            Value::Map(value) => value.serialize(serializer),
            Value::Float(value) => value.serialize(serializer),
            Value::Bytes(value) => value.serialize(serializer),
            Value::Bool(value) => value.serialize(serializer),
            Value::Unit(value) => value.serialize(serializer),
            Value::Null(value) => value.serialize(serializer),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> serde::de::Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("any valid lilliput value")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Bool(BoolValue::from(value)))
            }

            fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Int(IntValue::from(value)))
            }

            fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Int(IntValue::from(value)))
            }

            fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Int(IntValue::from(value)))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Int(IntValue::from(value)))
            }

            fn visit_i128<E>(self, _value: i128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::Other("i128 value"),
                    &self,
                ))
            }

            fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Int(IntValue::from(value)))
            }

            fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Int(IntValue::from(value)))
            }

            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Int(IntValue::from(value)))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Int(IntValue::from(value)))
            }

            fn visit_u128<E>(self, _value: u128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::Other("u128 value"),
                    &self,
                ))
            }

            fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Float(FloatValue::from(value)))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Float(FloatValue::from(value)))
            }

            fn visit_char<E>(self, value: char) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(value.encode_utf8(&mut [0u8; 4]))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(value.to_owned())
            }

            fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(value.to_owned())
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::String(StringValue::from(value)))
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_byte_buf(value.to_owned())
            }

            fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_byte_buf(value.to_owned())
            }

            fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Bytes(BytesValue::from(value)))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Null(NullValue))
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                serde::Deserialize::deserialize(deserializer)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Unit(UnitValue))
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                serde::Deserialize::deserialize(deserializer)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();

                while let Some(value) = seq.next_element()? {
                    values.push(value);
                }

                Ok(Value::Seq(SeqValue::from(values)))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                match map.next_key_seed(map::MapKeyClassifier)? {
                    Some(map::MapKeyClass::Map(first_key)) => {
                        let mut values = Map::new();

                        values.insert(first_key, map.next_value()?);
                        while let Some((key, value)) = map.next_entry()? {
                            values.insert(key, value);
                        }

                        Ok(Value::Map(MapValue::from(values)))
                    }
                    None => Ok(Value::Map(MapValue::default())),
                }
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::EnumAccess<'de>,
            {
                let _ = data;
                Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::Enum,
                    &self,
                ))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

#[doc(hidden)]
#[cfg(any(test, feature = "testing"))]
pub struct ValueArbitraryParameters {
    pub depth: u32,
    pub desired_size: u32,
    pub expected_branch_size: u32,
}

#[cfg(any(test, feature = "testing"))]
impl Default for ValueArbitraryParameters {
    fn default() -> Self {
        Self {
            // 4 levels deep
            depth: 4,
            // Shoot for maximum size of 128 nodes
            desired_size: 128,
            // We put up to 5 items per collection
            expected_branch_size: 5,
        }
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::arbitrary::Arbitrary for Value {
    type Parameters = ValueArbitraryParameters;
    type Strategy = proptest::prelude::BoxedStrategy<Value>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;

        let ValueArbitraryParameters {
            depth,
            desired_size,
            expected_branch_size,
        } = args;

        let leaf = prop_oneof![
            IntValue::arbitrary().prop_map(Value::Int),
            StringValue::arbitrary().prop_map(Value::String),
            FloatValue::arbitrary().prop_map(Value::Float),
            BytesValue::arbitrary().prop_map(Value::Bytes),
            BoolValue::arbitrary().prop_map(Value::Bool),
            UnitValue::arbitrary().prop_map(Value::Unit),
            NullValue::arbitrary().prop_map(Value::Null),
        ];

        let len: SizeRange = (0..(expected_branch_size as usize)).into();

        leaf.prop_recursive(depth, desired_size, expected_branch_size, move |inner| {
            prop_oneof![
                map::arbitrary_map_with(inner.clone(), inner.clone(), len.clone())
                    .prop_map(|map| Value::Map(map.into())),
                seq::arbitrary_seq_with(inner.clone(), len.clone())
                    .prop_map(|seq| Value::Seq(seq.into())),
            ]
        })
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn debug() {
        // Int
        assert_eq!(format!("{:?}", Value::Int(IntValue::default())), "0");
        assert_eq!(
            format!("{:#?}", Value::Int(IntValue::default())),
            "Int(\n    0_u8,\n)"
        );

        // String
        assert_eq!(
            format!("{:?}", Value::String(StringValue::default())),
            "\"\""
        );
        assert_eq!(
            format!("{:#?}", Value::String(StringValue::default())),
            "String(\n    \"\",\n)"
        );

        // Seq
        assert_eq!(format!("{:?}", Value::Seq(SeqValue::default())), "[]");
        assert_eq!(
            format!("{:#?}", Value::Seq(SeqValue::default())),
            "Seq(\n    [],\n)"
        );

        // Map
        assert_eq!(format!("{:?}", Value::Map(MapValue::default())), "{}");
        assert_eq!(
            format!("{:#?}", Value::Map(MapValue::default())),
            "Map(\n    {},\n)"
        );

        // Float
        assert_eq!(format!("{:?}", Value::Float(FloatValue::default())), "0.0");
        assert_eq!(
            format!("{:#?}", Value::Float(FloatValue::default())),
            "Float(\n    0.0_f32,\n)"
        );

        // Bytes
        assert_eq!(format!("{:?}", Value::Bytes(BytesValue::default())), "[]");
        assert_eq!(
            format!("{:#?}", Value::Bytes(BytesValue::default())),
            "Bytes(\n    [],\n)"
        );

        // Bool
        assert_eq!(format!("{:?}", Value::Bool(BoolValue::default())), "false");
        assert_eq!(
            format!("{:#?}", Value::Bool(BoolValue::default())),
            "Bool(\n    false,\n)"
        );

        // Null
        assert_eq!(format!("{:?}", Value::Null(NullValue)), "null");
        assert_eq!(
            format!("{:#?}", Value::Null(NullValue)),
            "Null(\n    null,\n)"
        );
    }
}
