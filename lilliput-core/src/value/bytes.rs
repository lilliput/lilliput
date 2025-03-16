use crate::binary::BytesSlice;

/// Represents a byte sequence.
///
/// # Binary representation
///
/// ```plain
/// 0b000001XX <INTEGER> [ BYTE, … ]
///   ├────┘├┘  └─ Length  └─ Bytes
///   │     └─ Length width exponent
///   └─ Bytes type
/// ```
///
/// The byte-width of the length value is obtained by:
///
/// ```plain
/// width = 2 ^ exponent
/// ```
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BytesValue(pub Vec<u8>);

impl BytesValue {
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for BytesValue {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a BytesValue> for &'a [u8] {
    fn from(value: &'a BytesValue) -> Self {
        &value.0
    }
}

impl From<BytesValue> for Vec<u8> {
    fn from(value: BytesValue) -> Self {
        value.0
    }
}

impl std::fmt::Debug for BytesValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&BytesSlice(&self.0), f)
    }
}

impl std::fmt::Display for BytesValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&BytesSlice(&self.0), f)
    }
}

#[cfg(test)]
impl proptest::prelude::Arbitrary for BytesValue {
    type Parameters = ();
    type Strategy = proptest::prelude::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy as _;
        proptest::collection::vec((u8::MIN)..(u8::MAX), 0..=10)
            .prop_map(Self)
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::BytesValue;

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", BytesValue::from(vec![1, 2, 3])),
            "[01, 02, 03]"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", BytesValue::from(vec![1, 2, 3])),
            "[00000001, 00000010, 00000011]"
        );

        assert_eq!(
            format!("{:#?}", BytesValue::from(vec![1, 2, 3])),
            "[0b00000001, 0b00000010, 0b00000011]"
        );
    }
}
