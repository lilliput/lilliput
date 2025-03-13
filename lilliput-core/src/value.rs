mod null;

pub use self::null::NullValue;

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ValueType {
    Null = 0b00000001,
    Reserved = 0b00000000,
}

impl ValueType {
    pub fn of(value: &Value) -> Self {
        match value {
            Value::Null(_) => ValueType::Null,
        }
    }

    pub fn detect(byte: u8) -> Self {
        match byte.leading_zeros() {
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
    /// Represents a null value.
    Null(NullValue),
}

impl Default for Value {
    fn default() -> Self {
        Self::Null(NullValue)
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
                Self::Null(value) => f.debug_tuple("Null").field(value).finish(),
            }
        } else {
            match self {
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
        // Null
        assert_eq!(format!("{:?}", Value::Null(NullValue)), "null");
        assert_eq!(
            format!("{:#?}", Value::Null(NullValue)),
            "Null(\n    null,\n)"
        );
    }
}
