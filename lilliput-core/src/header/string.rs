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
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StringHeader {
    Compact(CompactStringHeader),
    Extended(ExtendedStringHeader),
}

impl StringHeader {
    #[inline]
    pub fn new(len: usize, packing_mode: PackingMode) -> Self {
        if let Some(len) = Self::as_compact(len, packing_mode) {
            Self::compact(len)
        } else {
            Self::extended(len)
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

    #[inline]
    pub(crate) fn compact(len: u8) -> Self {
        Self::Compact(CompactStringHeader { len })
    }

    #[inline]
    pub(crate) fn extended(len: usize) -> Self {
        Self::Extended(ExtendedStringHeader { len })
    }

    fn as_compact(len: usize, packing_mode: PackingMode) -> Option<u8> {
        let allows_compact = packing_mode == PackingMode::Optimal;
        let mask = StringHeader::COMPACT_LEN_BITS as usize;
        let compact_len = len & mask;
        if allows_compact && compact_len == len {
            Some(compact_len as u8)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct CompactStringHeader {
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct ExtendedStringHeader {
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
}

#[cfg(any(test, feature = "testing"))]
impl proptest::prelude::Arbitrary for StringHeader {
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
        fn encode_decode_roundtrip(header in StringHeader::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_string_header(&header).unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_string_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
