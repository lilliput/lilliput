#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

use crate::config::PackingMode;

/// Header representing a sequence of values.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SeqHeader {
    /// Compact header.
    Compact(CompactSeqHeader),
    /// Extended header.
    Extended(ExtendedSeqHeader),
}

impl SeqHeader {
    /// Creates a compact header.
    #[inline]
    pub fn compact(len: u8) -> Self {
        assert!(len <= Self::COMPACT_LEN_MASK);

        Self::compact_unchecked(len)
    }

    /// Creates a compact header, without checking invariants.
    #[inline]
    pub fn compact_unchecked(len: u8) -> Self {
        Self::Compact(CompactSeqHeader { len })
    }

    /// Creates an extended header.
    #[inline]
    pub fn extended(len: usize) -> Self {
        Self::Extended(ExtendedSeqHeader { len })
    }

    /// Creates a header for a given sequence's length, for a given `packing_mode`.
    #[inline]
    pub fn for_len(len: usize, packing_mode: PackingMode) -> Self {
        if let Some(len) = Self::as_compact_len(len, packing_mode) {
            Self::compact_unchecked(len)
        } else {
            Self::extended(len)
        }
    }

    /// Returns `true` if the associated value has a length of zero, otherwise `false`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the associated value's length.
    pub fn len(&self) -> usize {
        match self {
            Self::Compact(compact) => compact.len().into(),
            Self::Extended(extended) => extended.len(),
        }
    }

    #[inline]
    fn as_compact_len(len: usize, packing_mode: PackingMode) -> Option<u8> {
        if packing_mode.is_optimal() && len <= (Self::COMPACT_MAX_LEN as usize) {
            Some(len as u8)
        } else {
            None
        }
    }
}

/// Compact header representing a sequence of values.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct CompactSeqHeader {
    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(strategy = "(0..=SeqHeader::COMPACT_MAX_LEN)")
    )]
    pub(crate) len: u8,
}

impl CompactSeqHeader {
    /// Returns `true` if the associated value has a length of zero, otherwise `false`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the associated value's length.
    pub fn len(&self) -> u8 {
        self.len
    }
}

/// Extended header representing a sequence of values.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct ExtendedSeqHeader {
    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(strategy = "super::arbitrary_len()")
    )]
    pub(crate) len: usize,
}

impl ExtendedSeqHeader {
    /// Returns `true` if the associated value has a length of zero, otherwise `false`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the associated value's length.
    pub fn len(&self) -> usize {
        self.len
    }
}

impl SeqHeader {
    pub(crate) const MASK: u8 = 0b00111111;
    pub(crate) const TYPE_BITS: u8 = 0b00100000;

    pub(crate) const COMPACT_VARIANT_BIT: u8 = 0b00010000;
    pub(crate) const COMPACT_LEN_MASK: u8 = 0b00001111;
    pub(crate) const EXTENDED_LEN_WIDTH_BITS: u8 = 0b00000111;

    pub(crate) const COMPACT_MAX_LEN: u8 = Self::COMPACT_LEN_MASK;
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

    #[test]
    fn compact_len_mask_is_correct_mask() {
        assert_eq!(SeqHeader::COMPACT_LEN_MASK, 0b00001111);
        assert_eq!(SeqHeader::COMPACT_MAX_LEN, 15);
    }

    #[test]
    fn compact_header_at_boundary_values() {
        assert_eq!(SeqHeader::compact(0).len(), 0);
        assert_eq!(SeqHeader::compact(1).len(), 1);
        assert_eq!(SeqHeader::compact(14).len(), 14);
        assert_eq!(SeqHeader::compact(15).len(), 15);
    }

    #[test]
    #[should_panic]
    fn compact_header_rejects_value_above_max() {
        SeqHeader::compact(16);
    }

    #[test]
    fn compact_unchecked_allows_boundary_values() {
        assert_eq!(SeqHeader::compact_unchecked(0).len(), 0);
        assert_eq!(SeqHeader::compact_unchecked(15).len(), 15);
    }

    #[test]
    fn for_len_uses_compact_up_to_max() {
        for len in 0..=15 {
            let header = SeqHeader::for_len(len, PackingMode::Optimal);
            assert!(matches!(header, SeqHeader::Compact(_)));
            assert_eq!(header.len(), len);
        }
    }

    #[test]
    fn for_len_uses_extended_above_max() {
        for len in 16..=20 {
            let header = SeqHeader::for_len(len, PackingMode::Optimal);
            assert!(matches!(header, SeqHeader::Extended(_)));
            assert_eq!(header.len(), len);
        }
    }

    proptest! {
        #[test]
        fn as_compact_len(len in usize::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let compact_len = SeqHeader::as_compact_len(len, packing_mode);
            let is_optimal = packing_mode == PackingMode::Optimal;
            let can_be_compact = len <= (SeqHeader::COMPACT_MAX_LEN as usize);

            if is_optimal && can_be_compact {
                prop_assert_eq!(compact_len, Some(len as u8));
            } else {
                prop_assert_eq!(compact_len, None);
            }
        }

        #[test]
        fn for_len(len in usize::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = SeqHeader::for_len(len, packing_mode);

            match packing_mode {
                PackingMode::None => {
                    prop_assert!(matches!(header, SeqHeader::Extended(_)));
                    prop_assert!(header.len() == len);
                },
                PackingMode::Native => {
                    prop_assert!(matches!(header, SeqHeader::Extended(_)));
                },
                PackingMode::Optimal => {
                    if len <= (SeqHeader::COMPACT_MAX_LEN as usize) {
                        prop_assert!(matches!(header, SeqHeader::Compact(_)));
                    } else {
                        prop_assert!(matches!(header, SeqHeader::Extended(_)));
                    }
                },
            }
        }

        #[test]
        fn encode_decode_roundtrip(header in SeqHeader::arbitrary(), config in EncoderConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_seq_header(&header).unwrap();

            prop_assert!(encoded.len() <= 1 + 8);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::from_reader(reader);
            let decoded = decoder.decode_seq_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }

        #[test]
        fn encode_decode_compact_boundary_values(len in 0u8..=SeqHeader::COMPACT_MAX_LEN, config in EncoderConfig::arbitrary()) {
            let header = SeqHeader::compact_unchecked(len);

            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_seq_header(&header).unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::from_reader(reader);
            let decoded = decoder.decode_seq_header().unwrap();

            prop_assert_eq!(decoded.len(), len as usize);
            prop_assert!(matches!(decoded, SeqHeader::Compact(_)));
        }
    }
}
