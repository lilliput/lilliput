#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

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
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
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

#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CompactIntHeader {
    pub(crate) is_signed: bool,
    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(strategy = "(0..=IntHeader::MAX_COMPACT_VALUE)")
    )]
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

#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ExtendedIntHeader {
    pub(crate) is_signed: bool,
    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(strategy = "(1..=IntHeader::MAX_EXTENDED_WIDTH)")
    )]
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
    pub const MAX_COMPACT_VALUE: u8 = Self::COMPACT_VALUE_BITS;
    pub const MAX_EXTENDED_WIDTH: u8 = Self::EXTENDED_WIDTH_BITS + 1;

    pub(crate) const TYPE_BITS: u8 = 0b10000000;

    pub(crate) const SIGNEDNESS_BIT: u8 = 0b00100000;

    pub(crate) const COMPACT_VARIANT_BIT: u8 = 0b01000000;
    pub(crate) const COMPACT_VALUE_BITS: u8 = 0b00011111;

    pub(crate) const EXTENDED_WIDTH_BITS: u8 = 0b00000111;
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
        num::zigzag::ToZigZag as _,
    };

    use super::*;

    proptest! {
        #[test]
        fn for_u8(unsigned in u8::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = IntHeader::for_unsigned(unsigned, packing_mode);

            let extended_width = header.extended_width().unwrap_or(0);

            match packing_mode {
                PackingMode::None => prop_assert!(extended_width == 1),
                PackingMode::Native => prop_assert!([1].contains(&extended_width)),
                PackingMode::Optimal => {
                    if unsigned <= IntHeader::COMPACT_VALUE_BITS {
                        prop_assert!(extended_width == 0)
                    } else {
                        prop_assert!(extended_width <= 1)
                    }
                },
            }
        }

        #[test]
        fn for_u16(unsigned in u16::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = IntHeader::for_unsigned(unsigned, packing_mode);

            let extended_width = header.extended_width().unwrap_or(0);

            match packing_mode {
                PackingMode::None => prop_assert!(extended_width == 2),
                PackingMode::Native => prop_assert!([1, 2].contains(&extended_width)),
                PackingMode::Optimal => {
                    if unsigned <= IntHeader::COMPACT_VALUE_BITS as u16 {
                        prop_assert!(extended_width == 0)
                    } else {
                        prop_assert!(extended_width <= 2)
                    }
                },
            }
        }

        #[test]
        fn for_u32(unsigned in u32::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = IntHeader::for_unsigned(unsigned, packing_mode);

            let extended_width = header.extended_width().unwrap_or(0);

            match packing_mode {
                PackingMode::None => prop_assert!(extended_width == 4),
                PackingMode::Native => prop_assert!([1, 2, 4].contains(&extended_width)),
                PackingMode::Optimal => {
                    if unsigned <= IntHeader::COMPACT_VALUE_BITS as u32 {
                        prop_assert!(extended_width == 0)
                    } else {
                        prop_assert!(extended_width <= 4)
                    }
                },
            }
        }

        #[test]
        fn for_u64(unsigned in u64::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = IntHeader::for_unsigned(unsigned, packing_mode);

            let extended_width = header.extended_width().unwrap_or(0);

            match packing_mode {
                PackingMode::None => prop_assert!(extended_width == 8),
                PackingMode::Native => prop_assert!([1, 2, 4, 8].contains(&extended_width)),
                PackingMode::Optimal => {
                    if unsigned <= IntHeader::COMPACT_VALUE_BITS as u64 {
                        prop_assert!(extended_width == 0)
                    } else {
                        prop_assert!(extended_width <= 8)
                    }
                },
            }
        }

        #[test]
        fn for_i8(signed in i8::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let unsigned = signed.to_zig_zag();
            let header = IntHeader::for_unsigned(unsigned, packing_mode);

            let extended_width = header.extended_width().unwrap_or(0);

            match packing_mode {
                PackingMode::None => prop_assert!(extended_width == 1),
                PackingMode::Native => prop_assert!([1].contains(&extended_width)),
                PackingMode::Optimal => {
                    if unsigned <= IntHeader::COMPACT_VALUE_BITS {
                        prop_assert!(extended_width == 0)
                    } else {
                        prop_assert!(extended_width <= 1)
                    }
                },
            }
        }

        #[test]
        fn for_i16(signed in i16::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let unsigned = signed.to_zig_zag();
            let header = IntHeader::for_unsigned(unsigned, packing_mode);

            let extended_width = header.extended_width().unwrap_or(0);

            match packing_mode {
                PackingMode::None => prop_assert!(extended_width == 2),
                PackingMode::Native => prop_assert!([1, 2].contains(&extended_width)),
                PackingMode::Optimal => {
                    if unsigned <= IntHeader::COMPACT_VALUE_BITS as u16 {
                        prop_assert!(extended_width == 0)
                    } else {
                        prop_assert!(extended_width <= 2)
                    }
                },
            }
        }

        #[test]
        fn for_i32(signed in i32::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let unsigned = signed.to_zig_zag();
            let header = IntHeader::for_unsigned(unsigned, packing_mode);

            let extended_width = header.extended_width().unwrap_or(0);

            match packing_mode {
                PackingMode::None => prop_assert!(extended_width == 4),
                PackingMode::Native => prop_assert!([1, 2, 4].contains(&extended_width)),
                PackingMode::Optimal => {
                    if unsigned <= IntHeader::COMPACT_VALUE_BITS as u32 {
                        prop_assert!(extended_width == 0)
                    } else {
                        prop_assert!(extended_width <= 4)
                    }
                },
            }
        }

        #[test]
        fn for_i64(signed in i64::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let unsigned = signed.to_zig_zag();
            let header = IntHeader::for_unsigned(unsigned, packing_mode);

            let extended_width = header.extended_width().unwrap_or(0);

            match packing_mode {
                PackingMode::None => prop_assert!(extended_width == 8),
                PackingMode::Native => prop_assert!([1, 2, 4, 8].contains(&extended_width)),
                PackingMode::Optimal => {
                    if unsigned <= IntHeader::COMPACT_VALUE_BITS as u64 {
                        prop_assert!(extended_width == 0)
                    } else {
                        prop_assert!(extended_width <= 8)
                    }
                },
            }
        }

        #[test]
        fn encode_decode_roundtrip(header in IntHeader::arbitrary(), config in EncoderConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_int_header(&header).unwrap();

            prop_assert!(encoded.len() == 1);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_int_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
