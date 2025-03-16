#[cfg(test)]
use proptest::prelude::*;

use crate::binary::{required_bytes_for_prim_int, Byte};

use super::{DecodeHeader, EncodeHeader, HeaderDecodeError, HeaderType};

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
pub enum MapHeader {
    Compact { len: usize },
    Extended { len_width: usize },
}

impl MapHeader {
    const TYPE_BITS: u8 = 0b00010000;

    const VARIANT_BIT: u8 = 0b00001000;

    const COMPACT_LEN_BITS: u8 = 0b00000111;

    const EXTENDED_LEN_WIDTH_BITS: u8 = 0b00000111;

    pub fn optimal(len: usize) -> Self {
        if Self::can_be_compact(len) {
            Self::Compact { len }
        } else {
            let len_width = required_bytes_for_prim_int(len);
            Self::extended(len_width)
        }
    }

    pub fn compact(len: usize) -> Self {
        assert!(Self::can_be_compact(len));

        Self::Compact { len }
    }

    pub fn extended(len_width: usize) -> Self {
        Self::Extended {
            len_width: len_width.max(1),
        }
    }

    fn can_be_compact(len: usize) -> bool {
        len <= (Self::COMPACT_LEN_BITS as usize)
    }
}

impl DecodeHeader for MapHeader {
    fn decode(byte: u8) -> Result<Self, HeaderDecodeError> {
        HeaderType::Map.validate(byte)?;

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

impl EncodeHeader for MapHeader {
    fn encode(self) -> u8 {
        let mut byte = Byte(Self::TYPE_BITS);

        match self {
            MapHeader::Compact { len } => {
                byte.set_bits(Self::VARIANT_BIT);

                let len_bits = Self::COMPACT_LEN_BITS;
                byte.set_bits_assert_masked_by(len as u8, len_bits);
            }
            MapHeader::Extended { len_width } => {
                let len_width_bits = Self::EXTENDED_LEN_WIDTH_BITS;
                byte.set_bits_assert_masked_by(len_width as u8 - 1, len_width_bits);
            }
        }

        byte.0
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for MapHeader {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;

        proptest::prop_oneof![
            (0..=7_usize).prop_map(Self::compact),
            (0..=100_usize).prop_map(Self::extended)
        ]
        .boxed()
    }
}
