use crate::config::PackingMode;

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
pub enum SeqHeader {
    Compact(CompactSeqHeader),
    Extended(ExtendedSeqHeader),
}

impl SeqHeader {
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
        Self::Compact(CompactSeqHeader { len })
    }

    #[inline]
    pub(crate) fn extended(len: usize) -> Self {
        Self::Extended(ExtendedSeqHeader { len })
    }

    fn as_compact(len: usize, packing_mode: PackingMode) -> Option<u8> {
        let allows_compact = packing_mode == PackingMode::Optimal;
        let mask = SeqHeader::COMPACT_LEN_BITS as usize;
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
pub struct CompactSeqHeader {
    pub(crate) len: u8,
}

impl CompactSeqHeader {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> u8 {
        self.len
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct ExtendedSeqHeader {
    pub(crate) len: usize,
}

impl ExtendedSeqHeader {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl SeqHeader {
    pub(crate) const TYPE_BITS: u8 = 0b00100000;

    pub(crate) const COMPACT_VARIANT_BIT: u8 = 0b00010000;
    pub(crate) const COMPACT_LEN_BITS: u8 = 0b00000111;
    pub(crate) const EXTENDED_LEN_WIDTH_BITS: u8 = 0b00000111;
}

#[cfg(any(test, feature = "testing"))]
impl proptest::prelude::Arbitrary for SeqHeader {
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
        fn encode_decode_roundtrip(header in SeqHeader::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_seq_header(&header).unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_seq_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
