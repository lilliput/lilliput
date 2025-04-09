use num_traits::{Signed, Unsigned};

use crate::{config::PackingMode, num::WithPackedBeBytes};

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
    #[inline]
    pub fn compact(is_signed: bool, bits: u8) -> Self {
        assert!(bits <= Self::COMPACT_VALUE_BITS);

        Self::Compact(CompactIntHeader { is_signed, bits })
    }

    #[inline]
    pub fn extended(is_signed: bool, width: u8) -> Self {
        assert!(width >= 1);
        assert!((width - 1) <= Self::EXTENDED_WIDTH_BITS);

        Self::Extended(ExtendedIntHeader { is_signed, width })
    }

    #[inline]
    pub fn for_signed<T>(value: T, packing_mode: PackingMode) -> Self
    where
        T: Signed + WithPackedBeBytes,
    {
        value.with_packed_be_bytes(packing_mode, |be_bytes| {
            Self::for_int_be_bytes(true, be_bytes, packing_mode)
        })
    }

    #[inline]
    pub fn for_unsigned<T>(value: T, packing_mode: PackingMode) -> Self
    where
        T: Unsigned + WithPackedBeBytes,
    {
        value.with_packed_be_bytes(packing_mode, |be_bytes| {
            Self::for_int_be_bytes(true, be_bytes, packing_mode)
        })
    }

    pub fn extended_width(&self) -> Option<u8> {
        match self {
            Self::Compact(_) => None,
            Self::Extended(header) => Some(header.width),
        }
    }

    #[inline]
    pub(crate) fn for_int_be_bytes(
        is_signed: bool,
        be_bytes: &[u8],
        packing_mode: PackingMode,
    ) -> Self {
        let width = be_bytes.len();

        let mut header = Self::Extended(ExtendedIntHeader {
            is_signed,
            width: width as u8,
        });

        if packing_mode == PackingMode::Optimal && width == 1 {
            let bits = be_bytes[width - 1];
            if bits <= Self::COMPACT_VALUE_BITS {
                header = Self::Compact(CompactIntHeader { is_signed, bits });
            }
        }

        header
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CompactIntHeader {
    pub(crate) is_signed: bool,
    pub(crate) bits: u8,
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
    pub(crate) is_signed: bool,
    pub(crate) width: u8,
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
    pub const MASK: u8 = 0b11111111;
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

        u8::arbitrary()
            .prop_map(|random_bits| {
                let is_signed: bool = random_bits & Self::SIGNEDNESS_BIT != 0b0;
                let is_compact: bool = random_bits & Self::COMPACT_VARIANT_BIT != 0b0;
                if is_compact {
                    let bits: u8 = random_bits & Self::COMPACT_VALUE_BITS;
                    Self::Compact(CompactIntHeader { is_signed, bits })
                } else {
                    let width: u8 = 1 + (random_bits & Self::EXTENDED_WIDTH_BITS);
                    Self::Extended(ExtendedIntHeader { is_signed, width })
                }
            })
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
