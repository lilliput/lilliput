#[cfg(test)]
use proptest::prelude::*;

use crate::{
    binary::Byte,
    num::{FromZigZag, ToZigZag},
};

use super::{DecodeHeader, EncodeHeader, HeaderDecodeError, HeaderType};

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
pub enum IntHeader {
    CompactSigned { value: i8 },
    CompactUnsigned { value: u8 },
    Extended { is_signed: bool, width: usize },
}

impl IntHeader {
    const TYPE_BITS: u8 = 0b10000000;

    const VARIANT_BIT: u8 = 0b01000000;
    const SIGNEDNESS_BIT: u8 = 0b00100000;

    const COMPACT_VALUE_BITS: u8 = 0b00011111;

    const EXTENDED_WIDTH_BITS: u8 = 0b00000111;

    pub fn compact_signed(value: i8) -> Self {
        Self::CompactSigned { value }
    }

    pub fn compact_unsigned(value: u8) -> Self {
        Self::CompactUnsigned { value }
    }

    pub fn extended(is_signed: bool, width: usize) -> Self {
        Self::Extended { is_signed, width }
    }

    pub fn extended_signed(width: usize) -> Self {
        Self::Extended {
            is_signed: true,
            width,
        }
    }

    pub fn extended_unsigned(width: usize) -> Self {
        Self::Extended {
            is_signed: false,
            width,
        }
    }

    pub fn is_signed(&self) -> bool {
        match self {
            Self::CompactSigned { .. } => true,
            Self::CompactUnsigned { .. } => false,
            Self::Extended { is_signed, .. } => *is_signed,
        }
    }
}

impl DecodeHeader for IntHeader {
    fn decode(byte: u8) -> Result<Self, HeaderDecodeError> {
        HeaderType::Int.validate(byte)?;

        let byte = Byte(byte);

        let is_signed = byte.contains_bits(Self::SIGNEDNESS_BIT);

        if byte.contains_bits(Self::VARIANT_BIT) {
            let unsigned = byte.masked_bits(Self::COMPACT_VALUE_BITS);
            if is_signed {
                let signed = i8::from_zig_zag(unsigned);
                Ok(Self::CompactSigned { value: signed })
            } else {
                Ok(Self::CompactUnsigned { value: unsigned })
            }
        } else {
            let width = byte.masked_bits(Self::EXTENDED_WIDTH_BITS) + 1;
            Ok(Self::Extended {
                is_signed,
                width: width.into(),
            })
        }
    }
}

impl EncodeHeader for IntHeader {
    fn encode(self) -> u8 {
        let mut byte = Byte(Self::TYPE_BITS);

        match self {
            IntHeader::CompactSigned { value } => {
                byte.set_bits(Self::VARIANT_BIT);
                byte.set_bits(Self::SIGNEDNESS_BIT);

                let value = value.to_zig_zag();
                let value_bits = Self::COMPACT_VALUE_BITS;
                byte.set_bits_assert_masked_by(value, value_bits);
            }
            IntHeader::CompactUnsigned { value } => {
                byte.set_bits(Self::VARIANT_BIT);

                let value_bits = Self::COMPACT_VALUE_BITS;
                byte.set_bits_assert_masked_by(value, value_bits);
            }
            IntHeader::Extended { is_signed, width } => {
                byte.set_bits_if(Self::SIGNEDNESS_BIT, is_signed);

                let width_bits = Self::EXTENDED_WIDTH_BITS;
                byte.set_bits_assert_masked_by(width as u8 - 1, width_bits);
            }
        }

        byte.0
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for IntHeader {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;

        proptest::prop_oneof![
            (-16..=15i8).prop_map(Self::compact_signed),
            (0..=31_u8).prop_map(Self::compact_unsigned),
            (1..=8_usize).prop_map(Self::extended_signed),
            (1..=8_usize).prop_map(Self::extended_unsigned),
        ]
        .boxed()
    }
}
