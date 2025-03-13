use std::hash::Hasher;

use std::hash::Hash;

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Copy, Clone)]
pub enum UnsignedIntValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl Default for UnsignedIntValue {
    fn default() -> Self {
        Self::U8(0)
    }
}

impl From<u8> for UnsignedIntValue {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

impl From<u16> for UnsignedIntValue {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl From<u32> for UnsignedIntValue {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u64> for UnsignedIntValue {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl PartialEq for UnsignedIntValue {
    fn eq(&self, other: &Self) -> bool {
        let lhs = match *self {
            Self::U8(value) => value as u64,
            Self::U16(value) => value as u64,
            Self::U32(value) => value as u64,
            Self::U64(value) => value as u64,
        };
        let rhs = match *other {
            Self::U8(value) => value as u64,
            Self::U16(value) => value as u64,
            Self::U32(value) => value as u64,
            Self::U64(value) => value as u64,
        };
        lhs == rhs
    }
}

impl PartialOrd for UnsignedIntValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for UnsignedIntValue {}

impl Ord for UnsignedIntValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.canonicalized().cmp(&other.canonicalized())
    }
}

impl Hash for UnsignedIntValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.canonicalized().hash(state)
    }
}

impl std::fmt::Debug for UnsignedIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self {
                Self::U8(value) => write!(f, "{value:#?}_u8"),
                Self::U16(value) => write!(f, "{value:#?}_u16"),
                Self::U32(value) => write!(f, "{value:#?}_u32"),
                Self::U64(value) => write!(f, "{value:#?}_u64"),
            }
        } else {
            match self {
                Self::U8(value) => std::fmt::Debug::fmt(value, f),
                Self::U16(value) => std::fmt::Debug::fmt(value, f),
                Self::U32(value) => std::fmt::Debug::fmt(value, f),
                Self::U64(value) => std::fmt::Debug::fmt(value, f),
            }
        }
    }
}

impl std::fmt::Display for UnsignedIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8(value) => std::fmt::Display::fmt(value, f),
            Self::U16(value) => std::fmt::Display::fmt(value, f),
            Self::U32(value) => std::fmt::Display::fmt(value, f),
            Self::U64(value) => std::fmt::Display::fmt(value, f),
        }
    }
}

impl UnsignedIntValue {
    pub(crate) fn canonicalized(&self) -> u64 {
        match *self {
            Self::U8(value) => value as u64,
            Self::U16(value) => value as u64,
            Self::U32(value) => value as u64,
            Self::U64(value) => value as u64,
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
        fn eq(lhs in u8::MIN..=u8::MAX, rhs in u8::MIN..=u8::MAX) {

            let lhs_values = [
                UnsignedIntValue::U8(lhs),
                UnsignedIntValue::U16(lhs as u16),
                UnsignedIntValue::U32(lhs as u32),
                UnsignedIntValue::U64(lhs as u64),
            ];

            let rhs_values = [
                UnsignedIntValue::U8(rhs),
                UnsignedIntValue::U16(rhs as u16),
                UnsignedIntValue::U32(rhs as u32),
                UnsignedIntValue::U64(rhs as u64),
            ];

            for lhs_value in &lhs_values {
                for rhs_value in &rhs_values {
                    let int_is_equal = lhs.eq(&rhs);
                    prop_assert_eq!(lhs_value.eq(&rhs_value), int_is_equal);
                    prop_assert_eq!(rhs_value.eq(&lhs_value), int_is_equal);
                }
            }
        }

        #[test]
        fn ord(lhs in u8::MIN..=u8::MAX, rhs in u8::MIN..=u8::MAX) {
            let lhs_values = [
                UnsignedIntValue::U8(lhs),
                UnsignedIntValue::U16(lhs as u16),
                UnsignedIntValue::U32(lhs as u32),
                UnsignedIntValue::U64(lhs as u64),
            ];

            let rhs_values = [
                UnsignedIntValue::U8(rhs),
                UnsignedIntValue::U16(rhs as u16),
                UnsignedIntValue::U32(rhs as u32),
                UnsignedIntValue::U64(rhs as u64),
            ];

            for lhs_value in &lhs_values {
                for rhs_value in &rhs_values {
                    let int_ordering = lhs.cmp(&rhs);
                    prop_assert_eq!(lhs_value.cmp(&rhs_value), int_ordering);
                    prop_assert_eq!(rhs_value.cmp(&lhs_value), int_ordering.reverse());
                }
            }
        }

        #[test]
        fn hash(lhs in u8::MIN..=u8::MAX) {
            use std::hash::BuildHasher as _;

            let values = [
                UnsignedIntValue::U8(lhs),
                UnsignedIntValue::U16(lhs as u16),
                UnsignedIntValue::U32(lhs as u32),
                UnsignedIntValue::U64(lhs as u64),
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
        assert_eq!(format!("{}", UnsignedIntValue::from(42_u8)), "42");
        assert_eq!(format!("{}", UnsignedIntValue::from(42_u16)), "42");
        assert_eq!(format!("{}", UnsignedIntValue::from(42_u32)), "42");
        assert_eq!(format!("{}", UnsignedIntValue::from(42_u64)), "42");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", UnsignedIntValue::from(42_u8)), "42");
        assert_eq!(format!("{:?}", UnsignedIntValue::from(42_u16)), "42");
        assert_eq!(format!("{:?}", UnsignedIntValue::from(42_u32)), "42");
        assert_eq!(format!("{:?}", UnsignedIntValue::from(42_u64)), "42");

        assert_eq!(format!("{:#?}", UnsignedIntValue::from(42_u8)), "42_u8");
        assert_eq!(format!("{:#?}", UnsignedIntValue::from(42_u16)), "42_u16");
        assert_eq!(format!("{:#?}", UnsignedIntValue::from(42_u32)), "42_u32");
        assert_eq!(format!("{:#?}", UnsignedIntValue::from(42_u64)), "42_u64");
    }
}
