use std::hash::{Hash, Hasher};

use decorum::{constraint::IsFloat, proxy::Constrained};

/// Represents a floating-point number.
///
/// # Binary representation
///
/// ```plain
/// 0b00001XXX <FLOAT>
///   ├───┘├─┘  └─ Value
///   │    └─ Width in bytes, minus 1
///   └─ Float Type
/// ```
#[derive(Copy, Clone)]
pub enum FloatValue {
    F32(f32),
    F64(f64),
}

impl FloatValue {
    pub(crate) const PREFIX_BIT: u8 = 0b00001000;

    pub(crate) const WIDTH_BITS: u8 = 0b00000111;
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
                FloatValue::F32(value) => write!(f, "{value:#?}_f32"),
                FloatValue::F64(value) => write!(f, "{value:#?}_f64"),
            }
        } else {
            match self {
                FloatValue::F32(value) => std::fmt::Debug::fmt(value, f),
                FloatValue::F64(value) => std::fmt::Debug::fmt(value, f),
            }
        }
    }
}

impl std::fmt::Display for FloatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatValue::F32(value) => std::fmt::Display::fmt(value, f),
            FloatValue::F64(value) => std::fmt::Display::fmt(value, f),
        }
    }
}

impl FloatValue {
    fn canonical_total(self) -> Constrained<f64, IsFloat> {
        decorum::Total::assert(self.as_f64())
    }
}

#[cfg(test)]
impl proptest::prelude::Arbitrary for FloatValue {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;

        proptest::prop_oneof![
            proptest::num::f32::ANY.prop_map(FloatValue::F32),
            proptest::num::f64::ANY.prop_map(FloatValue::F64),
        ]
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::FloatValue;

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
}
