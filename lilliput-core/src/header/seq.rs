#[cfg(test)]
use proptest::prelude::*;

use crate::{binary::Byte, num::int::CompactWidth as _};

use super::{DecodeHeader, EncodeHeader, Expectation, Marker};

/// Represents a sequence of values.
///
/// # Binary representation
///
/// ```plain
/// 0b001XXXXX <INTEGER>? [VALUE,*]
///   ├─┘│├──┘ ├───────┘  ├───────┘
///   │  ││    └─ Length? └─ Values
///   │  │└─ <depends on variant>
///   │  └─ Variant
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
/// ## Extended variant
///
/// ```plain
/// 0b00100XXX <INTEGER> [VALUE,*]
///   ├─┘││├─┘ ├───────┘ ├───────┘
///   │  │││   └─ Length └─ Values
///   │  ││└─ Width of length in bytes
///   │  │└─ Reserved bit
///   │  └─ Extended variant
///   └─ Seq type
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct SeqHeader {
    repr: SeqHeaderRepr,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SeqHeaderRepr {
    Compact { len: u8 },
    Extended { len_width: u8 },
}

impl SeqHeader {
    const TYPE_BITS: u8 = 0b00100000;

    const COMPACT_VARIANT_BIT: u8 = 0b00010000;
    const COMPACT_LEN_BITS: u8 = 0b00000111;
    const EXTENDED_LEN_WIDTH_BITS: u8 = 0b00000111;

    #[inline]
    pub fn optimal(len: usize) -> Self {
        if len <= (Self::COMPACT_LEN_BITS as usize) {
            Self::compact(len as u8)
        } else {
            Self::extended(len)
        }
    }

    #[inline]
    pub fn verbatim(_len: usize) -> Self {
        Self::from_repr(SeqHeaderRepr::Extended { len_width: 8_u8 })
    }

    #[inline]
    pub fn repr(&self) -> SeqHeaderRepr {
        self.repr
    }

    #[inline]
    pub fn extension_width(self) -> Option<usize> {
        match self.repr {
            SeqHeaderRepr::Compact { .. } => None,
            SeqHeaderRepr::Extended { len_width } => Some(len_width.into()),
        }
    }

    #[inline]
    pub(crate) fn from_repr(repr: SeqHeaderRepr) -> Self {
        Self::debug_assert_repr_valid(repr);

        Self { repr }
    }

    #[inline]
    fn compact(len: u8) -> Self {
        let len = Byte::assert_masked_by(len, Self::COMPACT_LEN_BITS);

        Self::from_repr(SeqHeaderRepr::Compact { len })
    }

    #[inline]
    pub fn extended(len: usize) -> Self {
        Self::from_repr(SeqHeaderRepr::Extended {
            len_width: len.compact_width(),
        })
    }

    #[inline]
    fn debug_assert_repr_valid(repr: SeqHeaderRepr) {
        match repr {
            SeqHeaderRepr::Compact { len } => {
                debug_assert!(len <= Self::COMPACT_LEN_BITS);
            }
            SeqHeaderRepr::Extended { len_width } => {
                debug_assert!(len_width - 1 <= Self::EXTENDED_LEN_WIDTH_BITS);
            }
        }
    }
}

impl DecodeHeader for SeqHeader {
    fn decode(byte: u8) -> Result<Self, Expectation<Marker>> {
        Marker::Seq.validate(byte)?;

        let byte = Byte(byte);

        let repr = if byte.contains_bits(Self::COMPACT_VARIANT_BIT) {
            let len = byte.masked_bits(Self::COMPACT_LEN_BITS);
            SeqHeaderRepr::Compact { len }
        } else {
            let len_width_bits = byte.masked_bits(Self::EXTENDED_LEN_WIDTH_BITS);
            SeqHeaderRepr::Extended {
                len_width: (len_width_bits + 1),
            }
        };

        Self::debug_assert_repr_valid(repr);

        Ok(Self::from_repr(repr))
    }
}

impl EncodeHeader for SeqHeader {
    fn encode(self) -> u8 {
        let mut byte = Byte(Self::TYPE_BITS);

        Self::debug_assert_repr_valid(self.repr);

        match self.repr {
            SeqHeaderRepr::Compact { len } => {
                byte.set_bits(Self::COMPACT_VARIANT_BIT);
                byte.set_bits_assert_masked_by(len, Self::COMPACT_LEN_BITS);
            }
            SeqHeaderRepr::Extended { len_width } => {
                byte.set_bits_assert_masked_by(len_width - 1, Self::EXTENDED_LEN_WIDTH_BITS);
            }
        }

        byte.0
    }
}

#[cfg(test)]
impl Arbitrary for SeqHeader {
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
        fn encode_decode_roundtrip(header in SeqHeader::arbitrary()) {
            let encoded = header.encode();
            let decoded = SeqHeader::decode(encoded).unwrap();

            prop_assert_eq!(&decoded, &header);
        }
    }
}
