mod bool;
mod bytes;
mod float;
mod map;
mod null;
mod seq;
mod string;

pub use self::{
    bool::BoolValue,
    bytes::BytesValue,
    float::FloatValue,
    map::{Map, MapValue},
    null::NullValue,
    seq::SeqValue,
    string::StringValue,
};

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ValueType {
    String = 0b01000000,
    Seq = 0b00100000,
    Map = 0b00010000,
    Float = 0b00001000,
    Bytes = 0b00000100,
    Bool = 0b00000010,
    Null = 0b00000001,
    Reserved = 0b00000000,
}

impl ValueType {
    pub fn of(value: &Value) -> Self {
        match value {
            Value::String(_) => ValueType::String,
            Value::Seq(_) => ValueType::Seq,
            Value::Map(_) => ValueType::Map,
            Value::Float(_) => ValueType::Float,
            Value::Bytes(_) => ValueType::Bytes,
            Value::Bool(_) => ValueType::Bool,
            Value::Null(_) => ValueType::Null,
        }
    }

    pub fn detect(byte: u8) -> Self {
        match byte.leading_zeros() {
            // 0b01000000
            1 => Self::String,
            // 0b00100000
            2 => Self::Seq,
            // 0b00010000
            3 => Self::Map,
            // 0b00001000
            4 => Self::Float,
            // 0b00000100
            5 => Self::Bytes,
            // 0b00000010
            6 => Self::Bool,
            // 0b00000001
            7 => Self::Null,
            // 0b00000000
            _ => Self::Reserved,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Value {
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

    /// Represents a null value.
    Null(NullValue),
}

impl Default for Value {
    fn default() -> Self {
        Self::Null(NullValue)
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

impl From<NullValue> for Value {
    fn from(value: NullValue) -> Self {
        Self::Null(value)
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self {
                Self::String(value) => f.debug_tuple("String").field(value).finish(),
                Self::Seq(value) => f.debug_tuple("Seq").field(value).finish(),
                Self::Map(value) => f.debug_tuple("Map").field(value).finish(),
                Self::Float(value) => f.debug_tuple("Float").field(value).finish(),
                Self::Bytes(value) => f.debug_tuple("Bytes").field(value).finish(),
                Self::Bool(value) => f.debug_tuple("Bool").field(value).finish(),
                Self::Null(value) => f.debug_tuple("Null").field(value).finish(),
            }
        } else {
            match self {
                Self::String(value) => std::fmt::Debug::fmt(value, f),
                Self::Seq(value) => std::fmt::Debug::fmt(value, f),
                Self::Map(value) => std::fmt::Debug::fmt(value, f),
                Self::Float(value) => std::fmt::Debug::fmt(value, f),
                Self::Bytes(value) => std::fmt::Debug::fmt(value, f),
                Self::Bool(value) => std::fmt::Debug::fmt(value, f),
                Self::Null(value) => std::fmt::Debug::fmt(value, f),
            }
        }
    }
}

impl Value {
    pub fn has_type(&self, value_type: ValueType) -> bool {
        ValueType::of(self) == value_type
    }
}

#[doc(hidden)]
#[cfg(test)]
pub struct ValueArbitraryParameters {
    pub depth: u32,
    pub desired_size: u32,
    pub expected_branch_size: u32,
}

#[cfg(test)]
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

#[cfg(test)]
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
            FloatValue::arbitrary().prop_map(Value::Float),
            BytesValue::arbitrary().prop_map(Value::Bytes),
            BoolValue::arbitrary().prop_map(Value::Bool),
            NullValue::arbitrary().prop_map(Value::Null),
        ];
        leaf.prop_recursive(depth, desired_size, expected_branch_size, |inner| {
            prop_oneof![
                prop::collection::hash_map(inner.clone(), inner.clone(), 0..10)
                    .prop_map(|hash_map| { Value::Map(MapValue::from(Map::from_iter(hash_map))) }),
                prop::collection::vec(inner, 0..10)
                    .prop_map(|vec| { Value::Seq(SeqValue::from(vec)) }),
            ]
        })
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {
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
