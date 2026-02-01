#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

use num_traits::{Signed, Unsigned};

use crate::{config::PackingMode, num::WithPackedBeBytes};

/// Header representing an integer number.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum IntHeader {
    /// Compact header.
    Compact(CompactIntHeader),
    /// Extended header.
    Extended(ExtendedIntHeader),
}

impl IntHeader {
    /// Creates a compact header.
    #[inline]
    pub fn compact(is_signed: bool, bits: u8) -> Self {
        assert!(bits <= Self::COMPACT_VALUE_BITS);

        Self::Compact(CompactIntHeader { is_signed, bits })
    }

    /// Creates an extended header.
    #[inline]
    pub fn extended(is_signed: bool, width: u8) -> Self {
        assert!(width >= 1);
        assert!((width - 1) <= Self::EXTENDED_WIDTH_BITS);

        Self::Extended(ExtendedIntHeader { is_signed, width })
    }

    /// Creates a header for a given signed `value`, for a given `packing_mode`.
    #[inline]
    pub fn for_signed<T>(value: T, packing_mode: PackingMode) -> Self
    where
        T: Signed + WithPackedBeBytes,
    {
        value.with_packed_be_bytes(packing_mode, |be_bytes| {
            Self::for_int_be_bytes(true, be_bytes, packing_mode)
        })
    }

    /// Creates a header for a given unsigned `value`, for a given `packing_mode`.
    #[inline]
    pub fn for_unsigned<T>(value: T, packing_mode: PackingMode) -> Self
    where
        T: Unsigned + WithPackedBeBytes,
    {
        value.with_packed_be_bytes(packing_mode, |be_bytes| {
            Self::for_int_be_bytes(false, be_bytes, packing_mode)
        })
    }

    /// Returns the extended byte-width, or `None` if compact.
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

/// Compact header representing an integer number.
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
    /// Returns the associated value's compact representation.
    pub fn bits(&self) -> u8 {
        self.bits
    }

    /// Returns `true`, if the associated value's type is signed, otherwise `false`.
    pub fn is_signed(&self) -> bool {
        self.is_signed
    }
}

/// Extended header representing an integer number.
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
    /// Returns the associated value's byte-width.
    pub fn width(&self) -> u8 {
        self.width
    }

    /// Returns `true`, if the associated value's type is signed, otherwise `false`.
    pub fn is_signed(&self) -> bool {
        self.is_signed
    }
}

impl IntHeader {
    pub(crate) const MASK: u8 = 0b11111111;
    pub(crate) const MAX_COMPACT_VALUE: u8 = Self::COMPACT_VALUE_BITS;
    pub(crate) const MAX_EXTENDED_WIDTH: u8 = Self::EXTENDED_WIDTH_BITS + 1;

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
        num::ToZigZag as _,
    };

    use super::*;

    #[test]
    fn signedness_bit_is_correct_mask() {
        assert_eq!(IntHeader::SIGNEDNESS_BIT, 0b00100000);
    }

    #[test]
    fn compact_signed_header_has_signedness_flag() {
        let header = IntHeader::compact(true, 15);
        match header {
            IntHeader::Compact(compact) => {
                assert!(compact.is_signed());
            }
            _ => panic!("Expected compact header"),
        }
    }

    #[test]
    fn compact_unsigned_header_lacks_signedness_flag() {
        let header = IntHeader::compact(false, 15);
        match header {
            IntHeader::Compact(compact) => {
                assert!(!compact.is_signed());
            }
            _ => panic!("Expected compact header"),
        }
    }

    #[test]
    fn extended_signed_header_has_signedness_flag() {
        let header = IntHeader::extended(true, 4);
        match header {
            IntHeader::Extended(extended) => {
                assert!(extended.is_signed());
            }
            _ => panic!("Expected extended header"),
        }
    }

    #[test]
    fn extended_unsigned_header_lacks_signedness_flag() {
        let header = IntHeader::extended(false, 4);
        match header {
            IntHeader::Extended(extended) => {
                assert!(!extended.is_signed());
            }
            _ => panic!("Expected extended header"),
        }
    }

    #[test]
    fn for_signed_u8_values_have_signed_flag() {
        for value in [0i8, 1, -1, 127, -128] {
            let header = IntHeader::for_signed(value, PackingMode::Optimal);
            match header {
                IntHeader::Compact(compact) => assert!(compact.is_signed()),
                IntHeader::Extended(extended) => assert!(extended.is_signed()),
            }
        }
    }

    #[test]
    fn for_unsigned_u8_values_lack_signed_flag() {
        for value in [0u8, 1, 31, 32, 127, 255] {
            let header = IntHeader::for_unsigned(value, PackingMode::Optimal);
            match header {
                IntHeader::Compact(compact) => assert!(!compact.is_signed()),
                IntHeader::Extended(extended) => assert!(!extended.is_signed()),
            }
        }
    }

    #[test]
    fn for_signed_i16_values_have_signed_flag() {
        for value in [0i16, 1, -1, 1000, -1000, i16::MAX, i16::MIN] {
            let header = IntHeader::for_signed(value, PackingMode::Optimal);
            match header {
                IntHeader::Compact(compact) => assert!(compact.is_signed()),
                IntHeader::Extended(extended) => assert!(extended.is_signed()),
            }
        }
    }

    #[test]
    fn for_unsigned_u16_values_lack_signed_flag() {
        for value in [0u16, 1, 31, 32, 1000, u16::MAX] {
            let header = IntHeader::for_unsigned(value, PackingMode::Optimal);
            match header {
                IntHeader::Compact(compact) => assert!(!compact.is_signed()),
                IntHeader::Extended(extended) => assert!(!extended.is_signed()),
            }
        }
    }

    #[test]
    fn for_signed_i32_values_have_signed_flag() {
        for value in [0i32, 1, -1, 100000, -100000, i32::MAX, i32::MIN] {
            let header = IntHeader::for_signed(value, PackingMode::Optimal);
            match header {
                IntHeader::Compact(compact) => assert!(compact.is_signed()),
                IntHeader::Extended(extended) => assert!(extended.is_signed()),
            }
        }
    }

    #[test]
    fn for_unsigned_u32_values_lack_signed_flag() {
        for value in [0u32, 1, 31, 32, 100000, u32::MAX] {
            let header = IntHeader::for_unsigned(value, PackingMode::Optimal);
            match header {
                IntHeader::Compact(compact) => assert!(!compact.is_signed()),
                IntHeader::Extended(extended) => assert!(!extended.is_signed()),
            }
        }
    }

    #[test]
    fn for_signed_i64_values_have_signed_flag() {
        for value in [0i64, 1, -1, 10000000000, -10000000000, i64::MAX, i64::MIN] {
            let header = IntHeader::for_signed(value, PackingMode::Optimal);
            match header {
                IntHeader::Compact(compact) => assert!(compact.is_signed()),
                IntHeader::Extended(extended) => assert!(extended.is_signed()),
            }
        }
    }

    #[test]
    fn for_unsigned_u64_values_lack_signed_flag() {
        for value in [0u64, 1, 31, 32, 10000000000, u64::MAX] {
            let header = IntHeader::for_unsigned(value, PackingMode::Optimal);
            match header {
                IntHeader::Compact(compact) => assert!(!compact.is_signed()),
                IntHeader::Extended(extended) => assert!(!extended.is_signed()),
            }
        }
    }

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
        fn for_signed_i8_has_signed_flag(value in i8::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = IntHeader::for_signed(value, packing_mode);
            match header {
                IntHeader::Compact(compact) => prop_assert!(compact.is_signed()),
                IntHeader::Extended(extended) => prop_assert!(extended.is_signed()),
            }
        }

        #[test]
        fn for_unsigned_u8_lacks_signed_flag(value in u8::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = IntHeader::for_unsigned(value, packing_mode);
            match header {
                IntHeader::Compact(compact) => prop_assert!(!compact.is_signed()),
                IntHeader::Extended(extended) => prop_assert!(!extended.is_signed()),
            }
        }

        #[test]
        fn for_signed_i16_has_signed_flag(value in i16::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = IntHeader::for_signed(value, packing_mode);
            match header {
                IntHeader::Compact(compact) => prop_assert!(compact.is_signed()),
                IntHeader::Extended(extended) => prop_assert!(extended.is_signed()),
            }
        }

        #[test]
        fn for_unsigned_u16_lacks_signed_flag(value in u16::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = IntHeader::for_unsigned(value, packing_mode);
            match header {
                IntHeader::Compact(compact) => prop_assert!(!compact.is_signed()),
                IntHeader::Extended(extended) => prop_assert!(!extended.is_signed()),
            }
        }

        #[test]
        fn for_signed_i32_has_signed_flag(value in i32::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = IntHeader::for_signed(value, packing_mode);
            match header {
                IntHeader::Compact(compact) => prop_assert!(compact.is_signed()),
                IntHeader::Extended(extended) => prop_assert!(extended.is_signed()),
            }
        }

        #[test]
        fn for_unsigned_u32_lacks_signed_flag(value in u32::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = IntHeader::for_unsigned(value, packing_mode);
            match header {
                IntHeader::Compact(compact) => prop_assert!(!compact.is_signed()),
                IntHeader::Extended(extended) => prop_assert!(!extended.is_signed()),
            }
        }

        #[test]
        fn for_signed_i64_has_signed_flag(value in i64::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = IntHeader::for_signed(value, packing_mode);
            match header {
                IntHeader::Compact(compact) => prop_assert!(compact.is_signed()),
                IntHeader::Extended(extended) => prop_assert!(extended.is_signed()),
            }
        }

        #[test]
        fn for_unsigned_u64_lacks_signed_flag(value in u64::arbitrary(), packing_mode in PackingMode::arbitrary()) {
            let header = IntHeader::for_unsigned(value, packing_mode);
            match header {
                IntHeader::Compact(compact) => prop_assert!(!compact.is_signed()),
                IntHeader::Extended(extended) => prop_assert!(!extended.is_signed()),
            }
        }

        #[test]
        fn encode_decode_preserves_signedness(is_signed in bool::arbitrary(), header_variant in bool::arbitrary(), config in EncoderConfig::arbitrary()) {
            let header = if header_variant {
                IntHeader::compact(is_signed, 15)
            } else {
                IntHeader::extended(is_signed, 4)
            };

            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_int_header(&header).unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::from_reader(reader);
            let decoded = decoder.decode_int_header().unwrap();

            match decoded {
                IntHeader::Compact(compact) => prop_assert_eq!(compact.is_signed(), is_signed),
                IntHeader::Extended(extended) => prop_assert_eq!(extended.is_signed(), is_signed),
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
            let mut decoder = Decoder::from_reader(reader);
            let decoded = decoder.decode_int_header().unwrap();
            prop_assert_eq!(&decoded, &header);
        }
    }
}
