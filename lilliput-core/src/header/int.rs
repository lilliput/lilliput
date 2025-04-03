use num_traits::{Signed, Unsigned};

use crate::{
    config::PackingMode,
    num::WithPackedBeBytes,
    value::{IntValue, SignedIntValue, UnsignedIntValue},
};

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
    Compact(CompactIntHeader),
    Extended(ExtendedIntHeader),
}

impl IntHeader {
    pub fn signed<T>(value: T, packing_mode: PackingMode) -> Self
    where
        T: Signed + WithPackedBeBytes,
    {
        let (header, _) = value.with_packed_be_bytes(packing_mode, |_, bytes| {
            Self::be_bytes(true, bytes, packing_mode)
        });

        header
    }

    pub fn unsigned<T>(value: T, packing_mode: PackingMode) -> Self
    where
        T: Unsigned + WithPackedBeBytes,
    {
        let (header, _) = value.with_packed_be_bytes(packing_mode, |_, bytes| {
            Self::be_bytes(false, bytes, packing_mode)
        });

        header
    }

    #[inline]
    pub fn new(value: IntValue, packing_mode: PackingMode) -> Self {
        match value {
            IntValue::Signed(value) => match value {
                SignedIntValue::I8(value) => Self::signed(value, packing_mode),
                SignedIntValue::I16(value) => Self::signed(value, packing_mode),
                SignedIntValue::I32(value) => Self::signed(value, packing_mode),
                SignedIntValue::I64(value) => Self::signed(value, packing_mode),
            },
            IntValue::Unsigned(value) => match value {
                UnsignedIntValue::U8(value) => Self::unsigned(value, packing_mode),
                UnsignedIntValue::U16(value) => Self::unsigned(value, packing_mode),
                UnsignedIntValue::U32(value) => Self::unsigned(value, packing_mode),
                UnsignedIntValue::U64(value) => Self::unsigned(value, packing_mode),
            },
        }
    }

    #[inline]
    pub(crate) fn be_bytes(
        is_signed: bool,
        be_bytes: &[u8],
        packing_mode: PackingMode,
    ) -> (Self, bool) {
        debug_assert!(be_bytes.len() <= 8);

        if let Some(bits) = Self::as_compact(be_bytes, packing_mode) {
            let header = Self::Compact(CompactIntHeader { is_signed, bits });

            (header, true)
        } else {
            let header = Self::Extended(ExtendedIntHeader {
                is_signed,
                width: be_bytes.len() as u8,
            });

            (header, false)
        }
    }

    pub fn extended_value_width(&self) -> Option<u8> {
        match self {
            Self::Compact(_) => None,
            Self::Extended(header) => Some(header.width),
        }
    }

    pub(crate) fn compact(is_signed: bool, bits: u8) -> Self {
        debug_assert!(bits <= IntHeader::COMPACT_VALUE_BITS);

        Self::Compact(CompactIntHeader { is_signed, bits })
    }

    pub(crate) fn extended(is_signed: bool, width: u8) -> Self {
        debug_assert!((width - 1) <= IntHeader::EXTENDED_WIDTH_BITS);

        Self::Extended(ExtendedIntHeader { is_signed, width })
    }

    pub(crate) fn as_compact(be_bytes: &[u8], packing_mode: PackingMode) -> Option<u8> {
        if packing_mode != PackingMode::Optimal {
            return None;
        }

        match be_bytes {
            &[bits] if bits <= IntHeader::COMPACT_VALUE_BITS => Some(bits),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CompactIntHeader {
    is_signed: bool,
    bits: u8,
}

impl CompactIntHeader {
    pub fn bits(&self) -> u8 {
        self.bits
    }

    pub fn is_signed(&self) -> bool {
        self.is_signed
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ExtendedIntHeader {
    is_signed: bool,
    width: u8,
}

impl ExtendedIntHeader {
    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn is_signed(&self) -> bool {
        self.is_signed
    }
}

impl IntHeader {
    pub(crate) const TYPE_BITS: u8 = 0b10000000;
    pub(crate) const SIGNEDNESS_BIT: u8 = 0b00100000;

    pub(crate) const COMPACT_VARIANT_BIT: u8 = 0b01000000;
    pub(crate) const COMPACT_VALUE_BITS: u8 = 0b00011111;

    pub(crate) const EXTENDED_WIDTH_BITS: u8 = 0b00000111;
}

#[cfg(any(test, feature = "testing"))]
impl proptest::arbitrary::Arbitrary for IntHeader {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;

        proptest::prop_oneof![
            (0..=Self::COMPACT_VALUE_BITS).prop_map(|bits| Self::compact(false, bits)),
            (0..=Self::COMPACT_VALUE_BITS).prop_map(|bits| Self::compact(true, bits)),
            (0..=Self::EXTENDED_WIDTH_BITS).prop_map(|bits| Self::extended(false, bits + 1)),
            (0..=Self::EXTENDED_WIDTH_BITS).prop_map(|bits| Self::extended(true, bits + 1)),
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
        fn encode_decode_roundtrip(header in IntHeader::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_int_header(&header).unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_int_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
