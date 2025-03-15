#[cfg(test)]
use proptest::{prelude::*, sample::SizeRange};

use super::Value;

/// Represents a sequence of values.
///
/// # Binary representation
///
/// ```plain
/// 0b001XXXXX <INTEGER>? [VALUE,*]
///   ├─┘│├──┘ ├───────┘  ├───────┘
///   │  ││    └─ Length? └─ Values
///   │  │└─ <depends on variant>
///   │  └─ Compactness
///   └─ Seq type
/// ```
///
/// ## Compact variant
///
/// ```plain
/// 0b0011XXXX [VALUE,*]
///   ├─┘│├──┘ ├───────┘
///   │  ││    └─ Values
///   │  │└─ Number of elements
///   │  └─ Compact variant
///   └─ Seq type
/// ```
///
/// ## Standard variant
///
/// ```plain
/// 0b00100XXX <INTEGER> [VALUE,*]
///   ├─┘││├─┘ ├───────┘ ├───────┘
///   │  │││   └─ Length └─ Values
///   │  ││└─ Width of length in bytes
///   │  │└─ Reserved bit
///   │  └─ Standard variant
///   └─ Seq type
/// ```
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SeqValue(pub Vec<Value>);

impl SeqValue {
    pub(crate) const PREFIX_BIT: u8 = 0b00100000;
    pub(crate) const COMPACTNESS_BIT: u8 = 0b00010000;

    pub(crate) const LONG_RESERVED_BIT: u8 = 0b00001000;
    pub(crate) const LONG_LEN_WIDTH_BITS: u8 = 0b00000111;
}

impl SeqValue {
    pub fn as_slice(&self) -> &[Value] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<Value> {
        self.0
    }
}

impl From<Vec<Value>> for SeqValue {
    fn from(value: Vec<Value>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a SeqValue> for &'a [Value] {
    fn from(value: &'a SeqValue) -> Self {
        &value.0
    }
}

impl From<SeqValue> for Vec<Value> {
    fn from(value: SeqValue) -> Self {
        value.0
    }
}

impl std::fmt::Debug for SeqValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

#[doc(hidden)]
#[cfg(test)]
pub struct SeqValueArbitraryParameters {
    pub items: BoxedStrategy<Value>,
    pub size: SizeRange,
}

#[cfg(test)]
impl Default for SeqValueArbitraryParameters {
    fn default() -> Self {
        Self {
            items: Value::arbitrary(),
            size: (0..10).into(),
        }
    }
}

#[cfg(test)]
impl Arbitrary for SeqValue {
    type Parameters = SeqValueArbitraryParameters;
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        proptest::collection::vec(args.items, args.size)
            .prop_map(Self)
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use crate::value::NullValue;

    use super::*;

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SeqValue::from(vec![Value::Null(NullValue)])),
            "[null]"
        );

        assert_eq!(
            format!("{:#?}", SeqValue::from(vec![Value::Null(NullValue)])),
            "[\n    Null(\n        null,\n    ),\n]"
        );
    }
}
