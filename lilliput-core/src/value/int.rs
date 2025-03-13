use std::hash::{Hash, Hasher};

mod signed;
mod unsigned;

pub use self::{signed::SignedIntValue, unsigned::UnsignedIntValue};

/// Represents an integer number.
///
/// # Binary representation
///
/// ```plain
/// 0b1XXXXXXX <INTEGER>?
///   │││├───┘
///   │││└─ <depends on variant>
///   ││└─ Signedness
///   │└─ Short variant / Long variant
///   └─ Integer type
/// ```
///
/// ## Short variant
///
/// ```plain
/// 0b10XXXXXX
///   │││├───┘
///   │││└─ Value
///   ││└─ Signedness
///   │└─ Short Variant
///   └─ Integer Type
/// ```
///
/// ## Long variant
///
/// ```plain
/// 0b11X00XXX <INTEGER>
///   │││├┘├─┘ ├───────┘
///   ││││ │   └─ Value
///   ││││ └─ Width
///   │││└─ Reserved bits
///   ││└─ Signedness
///   │└─ Long variant
///   └─ Integer type
/// ```
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Copy, Clone)]
pub enum IntValue {
    Signed(SignedIntValue),
    Unsigned(UnsignedIntValue),
}

impl IntValue {
    pub fn is_signed(&self) -> bool {
        match self {
            Self::Signed(_) => true,
            Self::Unsigned(_) => false,
        }
    }
}

impl IntValue {
    pub(crate) const PREFIX_BIT: u8 = 0b10000000;
    pub(crate) const VARIANT_BIT: u8 = 0b01000000;
    pub(crate) const SIGNEDNESS_BIT: u8 = 0b00100000;

    pub(crate) const LONG_RESERVED_BITS: u8 = 0b00011000;
    pub(crate) const LONG_WIDTH_BITS: u8 = 0b00000111;
}

impl Default for IntValue {
    fn default() -> Self {
        Self::Unsigned(Default::default())
    }
}

impl From<i8> for IntValue {
    fn from(value: i8) -> Self {
        Self::Signed(value.into())
    }
}

impl From<i16> for IntValue {
    fn from(value: i16) -> Self {
        Self::Signed(value.into())
    }
}

impl From<i32> for IntValue {
    fn from(value: i32) -> Self {
        Self::Signed(value.into())
    }
}

impl From<i64> for IntValue {
    fn from(value: i64) -> Self {
        Self::Signed(value.into())
    }
}

impl From<u8> for IntValue {
    fn from(value: u8) -> Self {
        Self::Unsigned(value.into())
    }
}

impl From<u16> for IntValue {
    fn from(value: u16) -> Self {
        Self::Unsigned(value.into())
    }
}

impl From<u32> for IntValue {
    fn from(value: u32) -> Self {
        Self::Unsigned(value.into())
    }
}

impl From<u64> for IntValue {
    fn from(value: u64) -> Self {
        Self::Unsigned(value.into())
    }
}

impl PartialEq for IntValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Signed(lhs), Self::Signed(rhs)) => lhs == rhs,
            (Self::Signed(lhs), Self::Unsigned(rhs)) => {
                let lhs = lhs.canonicalized();
                let rhs = rhs.canonicalized();

                if lhs.is_negative() {
                    false
                } else {
                    (lhs as u64) == rhs
                }
            }
            (Self::Unsigned(lhs), Self::Signed(rhs)) => {
                let lhs = lhs.canonicalized();
                let rhs = rhs.canonicalized();

                if rhs.is_negative() {
                    false
                } else {
                    lhs == (rhs as u64)
                }
            }
            (Self::Unsigned(lhs), Self::Unsigned(rhs)) => lhs == rhs,
        }
    }
}

impl PartialOrd for IntValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for IntValue {}

impl Ord for IntValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Unsigned(lhs), Self::Unsigned(rhs)) => lhs.cmp(rhs),
            (Self::Signed(lhs), Self::Signed(rhs)) => lhs.cmp(rhs),
            (Self::Unsigned(lhs), Self::Signed(rhs)) => {
                let lhs = lhs.canonicalized();
                let rhs = rhs.canonicalized();
                if rhs.is_negative() {
                    std::cmp::Ordering::Greater
                } else {
                    lhs.cmp(&(rhs as u64))
                }
            }
            (Self::Signed(lhs), Self::Unsigned(rhs)) => {
                let lhs = lhs.canonicalized();
                let rhs = rhs.canonicalized();
                if lhs.is_negative() {
                    std::cmp::Ordering::Less
                } else {
                    (lhs as u64).cmp(&rhs)
                }
            }
        }
    }
}

impl Hash for IntValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Self::Unsigned(value) => {
                let value = value.canonicalized();
                value.to_ne_bytes().hash(state)
            }
            Self::Signed(value) => {
                let value = value.canonicalized();
                if value.is_negative() {
                    value.to_ne_bytes().hash(state)
                } else {
                    (value as u64).to_ne_bytes().hash(state)
                }
            }
        }
    }
}

impl std::fmt::Debug for IntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Signed(value) => std::fmt::Debug::fmt(&value, f),
            Self::Unsigned(value) => std::fmt::Debug::fmt(&value, f),
        }
    }
}

impl std::fmt::Display for IntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Signed(value) => std::fmt::Display::fmt(value, f),
            Self::Unsigned(value) => std::fmt::Display::fmt(value, f),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::hash::RandomState;

    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn eq(signed in i8::MIN..=i8::MAX, unsigned in u8::MIN..=u8::MAX) {
            let signed_values = [
                IntValue::from(signed as i8),
                IntValue::from(signed as i16),
                IntValue::from(signed as i32),
                IntValue::from(signed as i64),
            ];

            let unsigned_values = [
                IntValue::from(unsigned),
                IntValue::from(unsigned as u16),
                IntValue::from(unsigned as u32),
                IntValue::from(unsigned as u64),
            ];

            // signed vs signed
            for lhs_value in &signed_values {
                for rhs_value in &signed_values {
                    prop_assert_eq!(lhs_value, rhs_value);
                    prop_assert_eq!(rhs_value, lhs_value);
                }
            }

            // unsigned vs unsigned
            for lhs_value in &unsigned_values {
                for rhs_value in &unsigned_values {
                    prop_assert_eq!(lhs_value, rhs_value);
                    prop_assert_eq!(rhs_value, lhs_value);
                }
            }

            // signed vs unsigned
            for lhs_value in &signed_values {
                for rhs_value in &unsigned_values {
                    if u8::try_from(signed) == Ok(unsigned) {
                        prop_assert_eq!(lhs_value, rhs_value);
                        prop_assert_eq!(rhs_value, lhs_value);
                    } else {
                        prop_assert_ne!(lhs_value, rhs_value);
                        prop_assert_ne!(rhs_value, lhs_value);
                    }
                }
            }
        }

        #[test]
        fn ord(signed in i8::MIN..=i8::MAX, unsigned in u8::MIN..=u8::MAX) {
            let signed_values = [
                IntValue::from(signed as i8),
                IntValue::from(signed as i16),
                IntValue::from(signed as i32),
                IntValue::from(signed as i64),
            ];

            let unsigned_values = [
                IntValue::from(unsigned),
                IntValue::from(unsigned as u16),
                IntValue::from(unsigned as u32),
                IntValue::from(unsigned as u64),
            ];

            // signed vs signed
            for lhs_value in &signed_values {
                for rhs_value in &signed_values {
                    let value_ordering = lhs_value.cmp(&rhs_value);
                    prop_assert_eq!(value_ordering, Ordering::Equal);
                }
            }

            // unsigned vs unsigned
            for lhs_value in &unsigned_values {
                for rhs_value in &unsigned_values {
                    let value_ordering = lhs_value.cmp(&rhs_value);
                    prop_assert_eq!(value_ordering, Ordering::Equal);
                }
            }

            // signed vs unsigned
            for lhs_value in &signed_values {
                for rhs_value in &unsigned_values {
                    let value_ordering = lhs_value.cmp(&rhs_value);

                    if let Ok(positive) = u8::try_from(signed) {
                        let int_ordering = positive.cmp(&unsigned);
                        prop_assert_eq!(value_ordering, int_ordering);
                    } else {
                        prop_assert_eq!(value_ordering, Ordering::Less);
                    }
                }
            }
        }

        #[test]
        fn hash(signed in i8::MIN..=i8::MAX, unsigned in u8::MIN..=u8::MAX) {
            use std::hash::BuildHasher as _;

            let signed_values = [
                IntValue::from(signed),
                IntValue::from(signed as i16),
                IntValue::from(signed as i32),
                IntValue::from(signed as i64),
            ];

            let unsigned_values = [
                IntValue::from(unsigned),
                IntValue::from(unsigned as u16),
                IntValue::from(unsigned as u32),
                IntValue::from(unsigned as u64),
            ];

            // signed vs signed
            for lhs_value in &signed_values {
                for rhs_value in &signed_values {
                    let build_hasher = RandomState::new();
                    let lhs_hash = build_hasher.hash_one(lhs_value);
                    let rhs_hash = build_hasher.hash_one(rhs_value);
                    prop_assert_eq!(lhs_hash, rhs_hash);
                }
            }

            // unsigned vs unsigned
            for lhs_value in &unsigned_values {
                for rhs_value in &unsigned_values {
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
        assert_eq!(format!("{}", IntValue::from(42_u8)), "42");
        assert_eq!(format!("{}", IntValue::from(42_u16)), "42");
        assert_eq!(format!("{}", IntValue::from(42_u32)), "42");
        assert_eq!(format!("{}", IntValue::from(42_u64)), "42");

        assert_eq!(format!("{}", IntValue::from(42_i8)), "42");
        assert_eq!(format!("{}", IntValue::from(42_i16)), "42");
        assert_eq!(format!("{}", IntValue::from(42_i32)), "42");
        assert_eq!(format!("{}", IntValue::from(42_i64)), "42");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", IntValue::from(42_u8)), "42");
        assert_eq!(format!("{:?}", IntValue::from(42_u16)), "42");
        assert_eq!(format!("{:?}", IntValue::from(42_u32)), "42");
        assert_eq!(format!("{:?}", IntValue::from(42_u64)), "42");

        assert_eq!(format!("{:?}", IntValue::from(42_i8)), "42");
        assert_eq!(format!("{:?}", IntValue::from(42_i16)), "42");
        assert_eq!(format!("{:?}", IntValue::from(42_i32)), "42");
        assert_eq!(format!("{:?}", IntValue::from(42_i64)), "42");

        assert_eq!(format!("{:#?}", IntValue::from(42_u8)), "42_u8");
        assert_eq!(format!("{:#?}", IntValue::from(42_u16)), "42_u16");
        assert_eq!(format!("{:#?}", IntValue::from(42_u32)), "42_u32");
        assert_eq!(format!("{:#?}", IntValue::from(42_u64)), "42_u64");

        assert_eq!(format!("{:#?}", IntValue::from(42_i8)), "42_i8");
        assert_eq!(format!("{:#?}", IntValue::from(42_i16)), "42_i16");
        assert_eq!(format!("{:#?}", IntValue::from(42_i32)), "42_i32");
        assert_eq!(format!("{:#?}", IntValue::from(42_i64)), "42_i64");
    }
}
