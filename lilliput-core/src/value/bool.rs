/// Represents a boolean.
///
/// # Binary representation
///
/// ```plain
/// 0b0000001X
///   ├─────┘└─ Value (0 = false, 1 = true)
///   └─ Data type
/// ```
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BoolValue(pub bool);

impl BoolValue {
    pub(crate) const PREFIX_BIT: u8 = 0b0000010;
    pub(crate) const VALUE_BIT: u8 = 0b0000001;
}

impl From<bool> for BoolValue {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<BoolValue> for bool {
    fn from(value: BoolValue) -> Self {
        value.0
    }
}

impl std::fmt::Debug for BoolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::fmt::Display for BoolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
impl proptest::prelude::Arbitrary for BoolValue {
    type Parameters = ();
    type Strategy = proptest::prelude::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy as _;
        proptest::bool::ANY.prop_map(Self).boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::BoolValue;

    #[test]
    fn display() {
        assert_eq!(format!("{}", BoolValue::from(false)), "false");
        assert_eq!(format!("{}", BoolValue::from(true)), "true");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", BoolValue::from(false)), "false");
        assert_eq!(format!("{:?}", BoolValue::from(true)), "true");

        assert_eq!(format!("{:#?}", BoolValue::from(false)), "false");
        assert_eq!(format!("{:#?}", BoolValue::from(true)), "true");
    }
}
