/// Represents a null value.
///
/// # Binary representation
///
/// ```plain
/// 0b00000001
///   ├──────┘
///   └─ Null type
/// ```
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NullValue;

impl std::fmt::Debug for NullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

impl std::fmt::Display for NullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

#[cfg(test)]
mod tests {
    use super::NullValue;

    #[test]
    fn display() {
        assert_eq!(format!("{}", NullValue), "null");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", NullValue), "null");
        assert_eq!(format!("{:#?}", NullValue), "null");
    }
}
