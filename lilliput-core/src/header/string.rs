#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

use crate::config::PackingMode;

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
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StringHeader {
    Compact(CompactStringHeader),
    Extended(ExtendedStringHeader),
}

impl StringHeader {
    #[inline]
    pub fn compact(len: u8) -> Self {
        assert!(len <= Self::COMPACT_LEN_BITS);

        Self::compact_unchecked(len)
    }

    #[inline]
    pub fn compact_unchecked(len: u8) -> Self {
        Self::Compact(CompactStringHeader { len })
    }

    #[inline]
    pub fn extended(len: usize) -> Self {
        Self::Extended(ExtendedStringHeader { len })
    }

    #[inline]
    pub fn for_len(len: usize, packing_mode: PackingMode) -> Self {
        if let Some(len) = Self::as_compact_len(len, packing_mode) {
            Self::compact_unchecked(len)
        } else {
            Self::extended(len)
        }
    }

    #[inline]
    pub fn as_compact_len(len: usize, packing_mode: PackingMode) -> Option<u8> {
        if packing_mode.is_optimal() && len <= Self::COMPACT_MAX_LEN as usize {
            Some(len as u8)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Compact(compact) => compact.len().into(),
            Self::Extended(extended) => extended.len(),
        }
    }
}

#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct CompactStringHeader {
    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(strategy = "(0..=StringHeader::COMPACT_MAX_LEN)")
    )]
    pub(crate) len: u8,
}

impl CompactStringHeader {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> u8 {
        self.len
    }
}

#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct ExtendedStringHeader {
    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(strategy = "super::arbitrary_len()")
    )]
    pub(crate) len: usize,
}

impl ExtendedStringHeader {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl StringHeader {
    pub const MASK: u8 = 0b01111111;
    pub(crate) const TYPE_BITS: u8 = 0b01000000;

    pub(crate) const COMPACT_VARIANT_BIT: u8 = 0b00100000;
    pub(crate) const COMPACT_LEN_BITS: u8 = 0b00011111;
    pub(crate) const EXTENDED_LEN_WIDTH_BITS: u8 = 0b00000111;

    #[allow(dead_code)]
    pub(crate) const COMPACT_MAX_LEN: u8 = Self::COMPACT_LEN_BITS;
    #[allow(dead_code)]
    pub(crate) const EXTENDED_MAX_LEN_WIDTH: u8 = 1 + Self::EXTENDED_LEN_WIDTH_BITS;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use test_log::test;

    use crate::{
        config::EncodingConfig,
        decoder::Decoder,
        encoder::Encoder,
        io::{SliceReader, VecWriter},
    };

    use super::*;

    proptest! {
        #[test]
        fn as_compact_len(len in usize::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let compact_len = StringHeader::as_compact_len(len, packing_mode);
            let is_optimal = packing_mode == PackingMode::Optimal;
            let can_be_compact = len <= (StringHeader::COMPACT_MAX_LEN as usize);

            if is_optimal && can_be_compact {
                prop_assert_eq!(compact_len, Some(len as u8));
            } else {
                prop_assert_eq!(compact_len, None);
            }
        }

        #[test]
        fn encode_decode_roundtrip(header in StringHeader::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_string_header(&header).unwrap();

            prop_assert!(encoded.len() <= 1 + 8);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_string_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
