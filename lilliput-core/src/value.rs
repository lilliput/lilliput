mod bool;
mod bytes;
mod float;
mod null;

pub use self::{bool::BoolValue, bytes::BytesValue, float::FloatValue, null::NullValue};

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ValueType {
    Float = 0b00001000,
    Bytes = 0b00000100,
    Bool = 0b00000010,
    Null = 0b00000001,
    Reserved = 0b00000000,
}

impl ValueType {
    pub fn of(value: &Value) -> Self {
        match value {
            Value::Float(_) => ValueType::Float,
            Value::Bytes(_) => ValueType::Bytes,
            Value::Bool(_) => ValueType::Bool,
            Value::Null(_) => ValueType::Null,
        }
    }

    pub fn detect(byte: u8) -> Self {
        match byte.leading_zeros() {
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

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Value {
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
                Self::Float(value) => f.debug_tuple("Float").field(value).finish(),
                Self::Bytes(value) => f.debug_tuple("Bytes").field(value).finish(),
                Self::Bool(value) => f.debug_tuple("Bool").field(value).finish(),
                Self::Null(value) => f.debug_tuple("Null").field(value).finish(),
            }
        } else {
            match self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {
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
