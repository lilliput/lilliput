#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;

use crate::binary::{trailing_non_zero_bytes, Byte};

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
pub enum StringHeader {
    Compact { len: usize },
    Extended { len_width: usize },
}

impl StringHeader {
    const TYPE_BITS: u8 = 0b01000000;

    const VARIANT_BIT: u8 = 0b00100000;
    const COMPACT_LEN_BITS: u8 = 0b00011111;
    const EXTENDED_LEN_WIDTH_BITS: u8 = 0b00000111;

    #[inline]
    pub fn optimal(len: usize) -> Self {
        if Self::can_be_compact(len) {
            Self::Compact { len }
        } else {
            Self::extended(len)
        }
    }

    #[inline]
    pub fn compact(len: usize) -> Self {
        assert!(Self::can_be_compact(len));

        Self::Compact { len }
    }

    #[inline]
    pub fn extended(len: usize) -> Self {
        Self::Extended {
            len_width: trailing_non_zero_bytes(len).max(1),
        }
    }

    fn can_be_compact(len: usize) -> bool {
        len <= (Self::COMPACT_LEN_BITS as usize)
    }
}

impl DecodeHeader for StringHeader {
    fn decode(byte: u8) -> Result<Self, Expectation<Marker>> {
        Marker::String.validate(byte)?;

        let byte = Byte(byte);

        if byte.contains_bits(Self::VARIANT_BIT) {
            let len = byte.masked_bits(Self::COMPACT_LEN_BITS);
            Ok(Self::Compact { len: len.into() })
        } else {
            let len_width = byte.masked_bits(Self::EXTENDED_LEN_WIDTH_BITS) + 1;
            Ok(Self::Extended {
                len_width: len_width.into(),
            })
        }
    }
}

impl EncodeHeader for StringHeader {
    fn encode(self) -> u8 {
        let mut byte = Byte(Self::TYPE_BITS);

        match self {
            StringHeader::Compact { len } => {
                byte.set_bits(Self::VARIANT_BIT);

                let len_bits = Self::COMPACT_LEN_BITS;
                byte.set_bits_assert_masked_by(len as u8, len_bits);
            }
            StringHeader::Extended { len_width } => {
                let len_width_bits = Self::EXTENDED_LEN_WIDTH_BITS;
                byte.set_bits_assert_masked_by(len_width as u8 - 1, len_width_bits);
            }
        }

        byte.0
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::arbitrary::Arbitrary for StringHeader {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy as _;
        proptest::prop_oneof![
            (0..=7_usize).prop_map(Self::compact),
            (0..=100_usize).prop_map(Self::extended)
        ]
        .boxed()
    }
}
