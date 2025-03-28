use crate::{binary::Byte, num::int::CompactWidth as _};

use super::{DecodeHeader, EncodeHeader, Expectation, Marker};

/// Represents a map of key-value pairs.
///
/// # Binary representation
///
/// ```plain
/// 0b0001XXXX <INTEGER>? [KEY:VALUE,*]
///   ├──┘│├─┘ ├───────┘  ├───────────┘
///   │   ││   └─ Length? └─ Key-value pairs
///   │   │└─ <depends on variant>
///   │   └─ Variant
///   └─ Map type
/// ```
///
/// ## Compact variant
///
/// ```plain
/// 0b00011XXX [KEY:VALUE,*]
///   ├──┘│├─┘ ├───────────┘
///   │   ││   └─ Key-value pairs
///   │   │└─ Length
///   │   └─ Compact variant
///   └─ Map type
/// ```
///
/// ## Extended variant
///
/// ```plain
/// 0b00010XXX <INTEGER> [KEY:VALUE,*]
///   ├──┘│├─┘ ├───────┘ ├───────────┘
///   │   ││   └─ Length └─ Key-value pairs
///   │   │└─ Number of bytes in length
///   │   └─ Extended variant
///   └─ Map type
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct MapHeader {
    repr: MapHeaderRepr,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MapHeaderRepr {
    Compact { len: u8 },
    Extended { len_width: u8 },
}

impl MapHeader {
    const TYPE_BITS: u8 = 0b00010000;

    const VARIANT_BIT: u8 = 0b00001000;

    const COMPACT_LEN_BITS: u8 = 0b00000111;

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
        Self::from_repr(MapHeaderRepr::Extended { len_width: 8_u8 })
    }

    #[inline]
    pub fn repr(&self) -> MapHeaderRepr {
        self.repr
    }

    #[inline]
    pub fn extension_width(self) -> Option<usize> {
        match self.repr {
            MapHeaderRepr::Compact { .. } => None,
            MapHeaderRepr::Extended { len_width } => Some(len_width.into()),
        }
    }

    #[inline]
    pub(crate) fn from_repr(repr: MapHeaderRepr) -> Self {
        Self::debug_assert_repr_valid(repr);

        Self { repr }
    }

    #[inline]
    fn compact(len: u8) -> Self {
        let len = Byte::assert_masked_by(len, Self::COMPACT_LEN_BITS);

        Self::from_repr(MapHeaderRepr::Compact { len })
    }

    #[inline]
    pub fn extended(len: usize) -> Self {
        Self::from_repr(MapHeaderRepr::Extended {
            len_width: len.compact_width(),
        })
    }

    #[inline]
    fn can_be_compact(len: usize) -> bool {
        len <= (Self::COMPACT_LEN_BITS as usize)
    }

    #[inline]
    fn debug_assert_repr_valid(repr: MapHeaderRepr) {
        match repr {
            MapHeaderRepr::Compact { len } => {
                debug_assert!(len <= Self::COMPACT_LEN_BITS);
            }
            MapHeaderRepr::Extended { len_width } => {
                debug_assert!(len_width - 1 <= Self::EXTENDED_LEN_WIDTH_BITS);
            }
        }
    }
}

impl DecodeHeader for MapHeader {
    fn decode(byte: u8) -> Result<Self, Expectation<Marker>> {
        Marker::Map.validate(byte)?;

        let byte = Byte(byte);

        let repr = if byte.contains_bits(Self::VARIANT_BIT) {
            let len = byte.masked_bits(Self::COMPACT_LEN_BITS);
            MapHeaderRepr::Compact { len }
        } else {
            let len_width_bits = byte.masked_bits(Self::EXTENDED_LEN_WIDTH_BITS);
            MapHeaderRepr::Extended {
                len_width: (len_width_bits + 1),
            }
        };

        Self::debug_assert_repr_valid(repr);

        Ok(Self::from_repr(repr))
    }
}

impl EncodeHeader for MapHeader {
    fn encode(self) -> u8 {
        let mut byte = Byte(Self::TYPE_BITS);

        Self::debug_assert_repr_valid(self.repr);

        match self.repr {
            MapHeaderRepr::Compact { len } => {
                byte.set_bits(Self::VARIANT_BIT);
                byte.set_bits_assert_masked_by(len, Self::COMPACT_LEN_BITS);
            }
            MapHeaderRepr::Extended { len_width } => {
                byte.set_bits_assert_masked_by(len_width - 1, Self::EXTENDED_LEN_WIDTH_BITS);
            }
        }

        byte.0
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::arbitrary::Arbitrary for MapHeader {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;

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
        fn encode_decode_roundtrip(header in MapHeader::arbitrary()) {
            let encoded = header.encode();
            let decoded = MapHeader::decode(encoded).unwrap();

            prop_assert_eq!(&decoded, &header);
        }
    }
}
