#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

use crate::config::PackingMode;

/// Represents a map of key-value pairs.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MapHeader {
    Compact(CompactMapHeader),
    Extended(ExtendedMapHeader),
}

impl MapHeader {
    #[inline]
    pub fn compact(len: u8) -> Self {
        assert!(len <= Self::COMPACT_LEN_BITS);

        Self::compact_unchecked(len)
    }

    #[inline]
    pub fn compact_unchecked(len: u8) -> Self {
        Self::Compact(CompactMapHeader { len })
    }

    #[inline]
    pub fn extended(len: usize) -> Self {
        Self::Extended(ExtendedMapHeader { len })
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
        if packing_mode.is_optimal() && len <= (Self::COMPACT_MAX_LEN as usize) {
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
pub struct CompactMapHeader {
    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(strategy = "(0..=MapHeader::COMPACT_MAX_LEN)")
    )]
    pub(crate) len: u8,
}

impl CompactMapHeader {
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
pub struct ExtendedMapHeader {
    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(strategy = "super::arbitrary_len()")
    )]
    pub(crate) len: usize,
}

impl ExtendedMapHeader {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl MapHeader {
    pub const MASK: u8 = 0b00011111;
    pub(crate) const TYPE_BITS: u8 = 0b00010000;

    pub(crate) const COMPACT_VARIANT_BIT: u8 = 0b00001000;
    pub(crate) const COMPACT_LEN_BITS: u8 = 0b00000111;

    pub(crate) const EXTENDED_LEN_WIDTH_BITS: u8 = 0b00000111;

    pub(crate) const COMPACT_MAX_LEN: u8 = Self::COMPACT_LEN_BITS;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use test_log::test;

    use crate::{
        config::EncoderConfig,
        decoder::Decoder,
        encoder::Encoder,
        io::{SliceReader, VecWriter},
    };

    use super::*;

    proptest! {
        #[test]
        fn as_compact_len(len in usize::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let compact_len = MapHeader::as_compact_len(len, packing_mode);

            if packing_mode.is_optimal() && len <= (MapHeader::COMPACT_MAX_LEN as usize) {
                prop_assert_eq!(compact_len, Some(len as u8));
            } else {
                prop_assert_eq!(compact_len, None);
            }
        }

        #[test]
        fn for_len(len in usize::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = MapHeader::for_len(len, packing_mode);

            match packing_mode {
                PackingMode::None => {
                    prop_assert!(matches!(header, MapHeader::Extended(_)));
                    prop_assert!(header.len() == len);
                },
                PackingMode::Native => {
                    prop_assert!(matches!(header, MapHeader::Extended(_)));
                },
                PackingMode::Optimal => {
                    if len <= (MapHeader::COMPACT_MAX_LEN as usize) {
                        prop_assert!(matches!(header, MapHeader::Compact(_)));
                    } else {
                        prop_assert!(matches!(header, MapHeader::Extended(_)));
                    }
                },
            }
        }

        #[test]
        fn encode_decode_roundtrip(header in MapHeader::arbitrary(), config in EncoderConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new_with_config(writer, config);
            encoder.encode_map_header(&header).unwrap();

            prop_assert!(encoded.len() <= 1 + 8);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_map_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
