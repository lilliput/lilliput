#[cfg(test)]
use proptest::prelude::*;

/// Represents a string.
///
/// # Binary representation
///
/// ```plain
/// 0b01XXXXXX
///   ├┘│├───┘
///   │ │└─ <depends on variant>
///   │ └─ Short variant / Long variant
///   └─ String Type
/// ```
///
/// ## Short variant
///
/// ```plain
/// 0b010XXXXX [CHAR,*]
///   ├┘│├───┘ ├──────┘
///   │ ││     └─ Characters
///   │ │└─ Length
///   │ └─ Short variant
///   └─ String type
/// ```
///
/// ## Long variant
///
/// ```plain
/// 0b01100XXX <INTEGER> [CHAR,*]
///   ├┘│├┘├─┘ ├───────┘ ├──────┘
///   │ ││ │   └─ Length └─ Characters
///   │ ││ └─ Number of bytes in <Length> - 1
///   │ │└─ Empty padding bits
///   │ └─ Long variant
///   └─ String type
/// ```
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StringValue(pub String);

impl StringValue {
    pub(crate) const PREFIX_BIT: u8 = 0b01000000;
    pub(crate) const VARIANT_BIT: u8 = 0b00100000;

    pub(crate) const LONG_RESERVED_BITS: u8 = 0b00011000;
    pub(crate) const LONG_LEN_WIDTH_BITS: u8 = 0b00000111;
}

impl From<String> for StringValue {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a StringValue> for &'a str {
    fn from(value: &'a StringValue) -> Self {
        &value.0
    }
}

impl From<StringValue> for String {
    fn from(value: StringValue) -> Self {
        value.0
    }
}

impl std::fmt::Debug for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.0)
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}

impl std::fmt::Display for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[doc(hidden)]
#[cfg(test)]
pub struct StringValueArbitraryParameters {}

#[cfg(test)]
impl Default for StringValueArbitraryParameters {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for StringValue {
    type Parameters = StringValueArbitraryParameters;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        let StringValueArbitraryParameters {} = args;

        proptest::string::string_regex("[a-zA-Z]+")
            .unwrap()
            .prop_map(StringValue::from)
            .boxed()
    }
}
