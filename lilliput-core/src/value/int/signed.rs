use std::hash::Hasher;

use std::hash::Hash;

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Copy, Clone)]
pub enum SignedIntValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}

impl Default for SignedIntValue {
    fn default() -> Self {
        Self::I8(0)
    }
}

impl From<i8> for SignedIntValue {
    fn from(value: i8) -> Self {
        Self::I8(value)
    }
}

impl From<i16> for SignedIntValue {
    fn from(value: i16) -> Self {
        Self::I16(value)
    }
}

impl From<i32> for SignedIntValue {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<i64> for SignedIntValue {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl PartialEq for SignedIntValue {
    fn eq(&self, other: &Self) -> bool {
        let lhs = match *self {
            Self::I8(value) => value as i64,
            Self::I16(value) => value as i64,
            Self::I32(value) => value as i64,
            Self::I64(value) => value,
        };
        let rhs = match *other {
            Self::I8(value) => value as i64,
            Self::I16(value) => value as i64,
            Self::I32(value) => value as i64,
            Self::I64(value) => value,
        };
        lhs == rhs
    }
}

impl PartialOrd for SignedIntValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SignedIntValue {}

impl Ord for SignedIntValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.canonicalized().cmp(&other.canonicalized())
    }
}

impl Hash for SignedIntValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.canonicalized().hash(state)
    }
}

impl std::fmt::Debug for SignedIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self {
                Self::I8(value) => write!(f, "{value:#?}_i8"),
                Self::I16(value) => write!(f, "{value:#?}_i16"),
                Self::I32(value) => write!(f, "{value:#?}_i32"),
                Self::I64(value) => write!(f, "{value:#?}_i64"),
            }
        } else {
            match self {
                Self::I8(value) => std::fmt::Debug::fmt(value, f),
                Self::I16(value) => std::fmt::Debug::fmt(value, f),
                Self::I32(value) => std::fmt::Debug::fmt(value, f),
                Self::I64(value) => std::fmt::Debug::fmt(value, f),
            }
        }
    }
}

impl std::fmt::Display for SignedIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8(value) => std::fmt::Display::fmt(value, f),
            Self::I16(value) => std::fmt::Display::fmt(value, f),
            Self::I32(value) => std::fmt::Display::fmt(value, f),
            Self::I64(value) => std::fmt::Display::fmt(value, f),
        }
    }
}

impl SignedIntValue {
    pub(crate) fn canonicalized(&self) -> i64 {
        match *self {
            Self::I8(value) => value as i64,
            Self::I16(value) => value as i64,
            Self::I32(value) => value as i64,
            Self::I64(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::hash::RandomState;

    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn eq(lhs in i8::MIN..=i8::MAX, rhs in i8::MIN..=i8::MAX) {

            let lhs_values = [
                SignedIntValue::I8(lhs),
                SignedIntValue::I16(lhs as i16),
                SignedIntValue::I32(lhs as i32),
                SignedIntValue::I64(lhs as i64),
            ];

            let rhs_values = [
                SignedIntValue::I8(rhs),
                SignedIntValue::I16(rhs as i16),
                SignedIntValue::I32(rhs as i32),
                SignedIntValue::I64(rhs as i64),
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
        fn ord(lhs in i8::MIN..=i8::MAX, rhs in i8::MIN..=i8::MAX) {
            let lhs_values = [
                SignedIntValue::I8(lhs),
                SignedIntValue::I16(lhs as i16),
                SignedIntValue::I32(lhs as i32),
                SignedIntValue::I64(lhs as i64),
            ];

            let rhs_values = [
                SignedIntValue::I8(rhs),
                SignedIntValue::I16(rhs as i16),
                SignedIntValue::I32(rhs as i32),
                SignedIntValue::I64(rhs as i64),
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
        fn hash(lhs in i8::MIN..=i8::MAX) {
            use std::hash::BuildHasher as _;

            let values = [
                SignedIntValue::I8(lhs),
                SignedIntValue::I16(lhs as i16),
                SignedIntValue::I32(lhs as i32),
                SignedIntValue::I64(lhs as i64),
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
        assert_eq!(format!("{}", SignedIntValue::from(42_i8)), "42");
        assert_eq!(format!("{}", SignedIntValue::from(42_i16)), "42");
        assert_eq!(format!("{}", SignedIntValue::from(42_i32)), "42");
        assert_eq!(format!("{}", SignedIntValue::from(42_i64)), "42");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SignedIntValue::from(42_i8)), "42");
        assert_eq!(format!("{:?}", SignedIntValue::from(42_i16)), "42");
        assert_eq!(format!("{:?}", SignedIntValue::from(42_i32)), "42");
        assert_eq!(format!("{:?}", SignedIntValue::from(42_i64)), "42");

        assert_eq!(format!("{:#?}", SignedIntValue::from(42_i8)), "42_i8");
        assert_eq!(format!("{:#?}", SignedIntValue::from(42_i16)), "42_i16");
        assert_eq!(format!("{:#?}", SignedIntValue::from(42_i32)), "42_i32");
        assert_eq!(format!("{:#?}", SignedIntValue::from(42_i64)), "42_i64");
    }
}
