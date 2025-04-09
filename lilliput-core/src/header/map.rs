use crate::config::PackingMode;

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
        if packing_mode.is_optimal() && len <= Self::COMPACT_MAX_LEN {
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct CompactMapHeader {
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct ExtendedMapHeader {
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

    pub(crate) const COMPACT_MAX_LEN: usize = Self::COMPACT_LEN_BITS as usize;
}

#[cfg(any(test, feature = "testing"))]
impl proptest::prelude::Arbitrary for MapHeader {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy as _;
        proptest::prop_oneof![
            (0..=Self::COMPACT_LEN_BITS).prop_map(Self::compact),
            proptest::num::u8::ANY.prop_map(|len| Self::extended(len as usize)),
            proptest::num::u16::ANY.prop_map(|len| Self::extended(len as usize)),
            proptest::num::u32::ANY.prop_map(|len| Self::extended(len as usize)),
            proptest::num::u64::ANY.prop_map(|len| Self::extended(len as usize)),
        ]
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::{
        config::EncodingConfig,
        decoder::Decoder,
        encoder::Encoder,
        io::{SliceReader, VecWriter},
    };

    use super::*;

    proptest! {
        #[test]
        fn encode_decode_roundtrip(header in MapHeader::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_map_header(&header).unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_map_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
