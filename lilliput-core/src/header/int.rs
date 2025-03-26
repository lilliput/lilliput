use num_traits::{ToBytes, Unsigned};

use crate::{binary::Byte, num::int::CompactWidth};

use super::{DecodeHeader, EncodeHeader, Expectation, Marker};

/// Represents an integer number.
///
/// # Binary representation
///
/// ```plain
/// 0b1XXXXXXX <INTEGER>?
///   │││├───┘
///   │││└─ <depends on variant>
///   ││└─ Signedness
///   │└─ Variant
///   └─ Integer type
/// ```
///
/// ## Short variant
///
/// ```plain
/// 0b11XXXXXX
///   │││├───┘
///   │││└─ Value
///   ││└─ Signedness
///   │└─ Compact variant
///   └─ Integer type
/// ```
///
/// ## Long variant
///
/// ```plain
/// 0b10X00XXX <INTEGER>
///   │││├┘├─┘ ├───────┘
///   ││││ │   └─ Value
///   ││││ └─ Width
///   │││└─ Reserved bits
///   ││└─ Signedness
///   │└─ Extended variant
///   └─ Integer type
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct IntHeader {
    repr: IntHeaderRepr,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum IntHeaderRepr {
    Compact { is_signed: bool, bits: u8 },
    Extended { is_signed: bool, width: u8 },
}

impl IntHeader {
    const TYPE_BITS: u8 = 0b10000000;

    const VARIANT_BIT: u8 = 0b01000000;
    const SIGNEDNESS_BIT: u8 = 0b00100000;

    const COMPACT_VALUE_BITS: u8 = 0b00011111;

    const EXTENDED_WIDTH_BITS: u8 = 0b00000111;

    #[inline]
    pub fn optimal<T, const N: usize>(is_signed: bool, bits: T) -> Self
    where
        T: Unsigned + CompactWidth + PartialOrd + From<u8> + ToBytes<Bytes = [u8; N]>,
    {
        if bits <= T::from(Self::COMPACT_VALUE_BITS) {
            let bytes: [u8; N] = bits.to_be_bytes();
            let bits = bytes[N - 1] & Self::COMPACT_VALUE_BITS;
            Self::compact(is_signed, bits)
        } else {
            Self::extended(is_signed, bits.compact_width())
        }
    }

    #[inline]
    pub fn verbatim<T, const N: usize>(is_signed: bool, _value: T) -> Self
    where
        T: ToBytes<Bytes = [u8; N]>,
    {
        Self::from_repr(IntHeaderRepr::Extended {
            is_signed,
            width: N as u8,
        })
    }

    #[inline]
    pub fn repr(&self) -> IntHeaderRepr {
        self.repr
    }

    #[inline]
    pub fn extension_width(self) -> Option<usize> {
        match self.repr {
            IntHeaderRepr::Compact { .. } => None,
            IntHeaderRepr::Extended { width, .. } => Some(width.into()),
        }
    }

    #[inline]
    pub(crate) fn from_repr(repr: IntHeaderRepr) -> Self {
        Self::debug_assert_repr_valid(repr);

        match repr {
            IntHeaderRepr::Compact { bits, .. } => {
                debug_assert!(bits <= Self::COMPACT_VALUE_BITS);
            }
            IntHeaderRepr::Extended { width, .. } => {
                debug_assert!(width - 1 <= Self::EXTENDED_WIDTH_BITS);
            }
        }

        Self { repr }
    }

    #[inline]
    fn compact(is_signed: bool, bits: u8) -> Self {
        let bits = Byte::assert_masked_by(bits, Self::COMPACT_VALUE_BITS);

        Self::from_repr(IntHeaderRepr::Compact { is_signed, bits })
    }

    #[inline]
    fn extended(is_signed: bool, width: u8) -> Self {
        let width = Byte::assert_masked_by(width - 1, Self::EXTENDED_WIDTH_BITS) + 1;

        Self::from_repr(IntHeaderRepr::Extended { is_signed, width })
    }

    #[inline(always)]
    fn debug_assert_repr_valid(repr: IntHeaderRepr) {
        match repr {
            IntHeaderRepr::Compact { bits, .. } => {
                debug_assert!(bits <= Self::COMPACT_VALUE_BITS);
            }
            IntHeaderRepr::Extended { width, .. } => {
                debug_assert!(width - 1 <= Self::EXTENDED_WIDTH_BITS);
            }
        }
    }

    #[inline]
    pub fn is_signed(self) -> bool {
        match self.repr {
            IntHeaderRepr::Compact { is_signed, .. } => is_signed,
            IntHeaderRepr::Extended { is_signed, .. } => is_signed,
        }
    }
}

impl DecodeHeader for IntHeader {
    fn decode(byte: u8) -> Result<Self, Expectation<Marker>> {
        Marker::Int.validate(byte)?;

        let byte = Byte(byte);

        let is_signed = byte.contains_bits(Self::SIGNEDNESS_BIT);

        let repr = if byte.contains_bits(Self::VARIANT_BIT) {
            let bits = byte.masked_bits(Self::COMPACT_VALUE_BITS);
            IntHeaderRepr::Compact { is_signed, bits }
        } else {
            let width_bits = byte.masked_bits(Self::EXTENDED_WIDTH_BITS);
            IntHeaderRepr::Extended {
                is_signed,
                width: width_bits + 1,
            }
        };

        Self::debug_assert_repr_valid(repr);

        Ok(Self::from_repr(repr))
    }
}

impl EncodeHeader for IntHeader {
    fn encode(self) -> u8 {
        let mut byte = Byte(Self::TYPE_BITS);

        Self::debug_assert_repr_valid(self.repr);

        match self.repr {
            IntHeaderRepr::Compact { is_signed, bits } => {
                byte.set_bits(Self::VARIANT_BIT);
                byte.set_bits_if(Self::SIGNEDNESS_BIT, is_signed);
                byte.set_bits_assert_masked_by(bits, Self::COMPACT_VALUE_BITS);
            }
            IntHeaderRepr::Extended { is_signed, width } => {
                byte.set_bits_if(Self::SIGNEDNESS_BIT, is_signed);
                byte.set_bits_assert_masked_by(width - 1, Self::EXTENDED_WIDTH_BITS);
            }
        }

        byte.0
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::arbitrary::Arbitrary for IntHeader {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;

        proptest::prop_oneof![
            (0..=31_u8).prop_map(|bits| Self::compact(true, bits)),
            (0..=31_u8).prop_map(|bits| Self::compact(false, bits)),
            (1..=8_u8).prop_map(|width| Self::extended(true, width)),
            (1..=8_u8).prop_map(|width| Self::extended(false, width)),
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
        fn encode_decode_roundtrip(header in IntHeader::arbitrary()) {
            let encoded = header.encode();
            let decoded = IntHeader::decode(encoded).unwrap();

            prop_assert_eq!(&decoded, &header);
        }
    }
}
