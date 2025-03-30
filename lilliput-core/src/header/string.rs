use crate::{binary::Byte, num::int::CompactWidth as _};

use super::{DecodeHeader, EncodeHeader, Expectation, Marker};

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
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct StringHeader {
    repr: StringHeaderRepr,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StringHeaderRepr {
    Compact { len: u8 },
    Extended { len_width: u8 },
}

impl StringHeader {
    const TYPE_BITS: u8 = 0b01000000;

    const COMPACT_VARIANT_BIT: u8 = 0b00100000;
    const COMPACT_LEN_BITS: u8 = 0b00011111;
    const EXTENDED_LEN_WIDTH_BITS: u8 = 0b00000111;

    #[inline]
    pub fn optimal(len: usize) -> Self {
        if Self::can_be_compact(len) {
            Self::compact(len as u8)
        } else {
            Self::extended(len)
        }
    }

    #[inline]
    pub fn verbatim(_len: usize) -> Self {
        Self::from_repr(StringHeaderRepr::Extended { len_width: 8_u8 })
    }

    #[inline]
    pub fn repr(&self) -> StringHeaderRepr {
        self.repr
    }

    #[inline]
    pub fn extension_width(self) -> Option<usize> {
        match self.repr {
            StringHeaderRepr::Compact { .. } => None,
            StringHeaderRepr::Extended { len_width } => Some(len_width.into()),
        }
    }

    #[inline]
    pub(crate) fn from_repr(repr: StringHeaderRepr) -> Self {
        Self::debug_assert_repr_valid(repr);

        Self { repr }
    }

    #[inline]
    fn compact(len: u8) -> Self {
        let len = Byte::assert_masked_by(len, Self::COMPACT_LEN_BITS);

        Self::from_repr(StringHeaderRepr::Compact { len })
    }

    #[inline]
    fn extended(len: usize) -> Self {
        Self::from_repr(StringHeaderRepr::Extended {
            len_width: len.compact_width(),
        })
    }

    #[inline]
    fn can_be_compact(len: usize) -> bool {
        len <= (Self::COMPACT_LEN_BITS as usize)
    }

    #[inline]
    fn debug_assert_repr_valid(repr: StringHeaderRepr) {
        match repr {
            StringHeaderRepr::Compact { len } => {
                debug_assert!(len <= Self::COMPACT_LEN_BITS);
            }
            StringHeaderRepr::Extended { len_width } => {
                debug_assert!(len_width - 1 <= Self::EXTENDED_LEN_WIDTH_BITS);
            }
        }
    }
}

impl DecodeHeader for StringHeader {
    fn decode(byte: u8) -> Result<Self, Expectation<Marker>> {
        Marker::String.validate(byte)?;

        let byte = Byte(byte);

        let repr = if byte.contains_bits(Self::COMPACT_VARIANT_BIT) {
            let len = byte.masked_bits(Self::COMPACT_LEN_BITS);
            StringHeaderRepr::Compact { len }
        } else {
            let len_width_bits = byte.masked_bits(Self::EXTENDED_LEN_WIDTH_BITS);
            StringHeaderRepr::Extended {
                len_width: (len_width_bits + 1),
            }
        };

        Self::debug_assert_repr_valid(repr);

        Ok(Self::from_repr(repr))
    }
}

impl EncodeHeader for StringHeader {
    fn encode(self) -> u8 {
        let mut byte = Byte(Self::TYPE_BITS);

        Self::debug_assert_repr_valid(self.repr);

        match self.repr {
            StringHeaderRepr::Compact { len } => {
                byte.set_bits(Self::COMPACT_VARIANT_BIT);

                let len_bits = Self::COMPACT_LEN_BITS;
                byte.set_bits_assert_masked_by(len, len_bits);
            }
            StringHeaderRepr::Extended { len_width } => {
                let len_width_bits = Self::EXTENDED_LEN_WIDTH_BITS;
                byte.set_bits_assert_masked_by(len_width - 1, len_width_bits);
            }
        }

        byte.0
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::arbitrary::Arbitrary for StringHeader {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy as _;
        proptest::prop_oneof![
            (0..=7_u8).prop_map(Self::compact),
            (0..=100_usize).prop_map(Self::extended)
        ]
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn encode_decode_roundtrip(header in StringHeader::arbitrary()) {
            let encoded = header.encode();
            let decoded = StringHeader::decode(encoded).unwrap();

            prop_assert_eq!(&decoded, &header);
        }
    }
}
