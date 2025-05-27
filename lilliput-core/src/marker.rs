//! Value type markers.

use crate::{
    error::Expectation,
    header::{
        BoolHeader, BytesHeader, FloatHeader, IntHeader, MapHeader, NullHeader, SeqHeader,
        StringHeader, UnitHeader,
    },
};

/// A value's type marker.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum Marker {
    /// Integer values.
    Int = 0b10000000,
    /// String values.
    String = 0b01000000,
    /// Sequence values.
    Seq = 0b00100000,
    /// Map values.
    Map = 0b00010000,
    /// Floating-point values.
    Float = 0b00001000,
    /// Byte array values.
    Bytes = 0b00000100,
    /// Bool values.
    Bool = 0b00000010,
    /// Unit values.
    Unit = 0b00000001,
    /// Null values.
    Null = 0b00000000,
}

impl std::fmt::Display for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "integer"),
            Self::String => write!(f, "string"),
            Self::Seq => write!(f, "sequence"),
            Self::Map => write!(f, "map"),
            Self::Float => write!(f, "float"),
            Self::Bytes => write!(f, "byte sequence"),
            Self::Bool => write!(f, "bool"),
            Self::Null => write!(f, "null"),
            Self::Unit => write!(f, "unit"),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::de::Expected for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Marker {
    /// Detects a value's type from its header byte.
    #[inline]
    pub fn detect(byte: u8) -> Self {
        // Safety: The following is safe because:
        // - the value returned by `Self::repr_for(byte)` is
        //   guaranteed to contain at most a single non-zero bit.
        // - `Marker` is `#[repr(u8)]`, and covers each possible `repr`.
        //
        // This unsafe cast directly from the repr provided
        // a 14% performance boost compared to a safe match:
        //
        // ```
        // match byte.leading_zeros() {
        //     0 => Self::Int,
        //     // ...
        //     8 => Self::Reserved,
        // }
        // ```
        unsafe { std::mem::transmute_copy(&Self::repr_for(byte)) }
    }

    /// Returns a given mask's bit-mask.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn header_mask(&self) -> u8 {
        match self {
            Self::Int => IntHeader::MASK,
            Self::String => StringHeader::MASK,
            Self::Seq => SeqHeader::MASK,
            Self::Map => MapHeader::MASK,
            Self::Float => FloatHeader::MASK,
            Self::Bytes => BytesHeader::MASK,
            Self::Bool => BoolHeader::MASK,
            Self::Unit => UnitHeader::MASK,
            Self::Null => NullHeader::MASK,
        }
    }

    #[inline]
    fn repr_for(byte: u8) -> u8 {
        let leading_zeros = byte.leading_zeros();
        0b10000000_u8.checked_shr(leading_zeros).unwrap_or_default()
    }

    /// Validates a given header `byte`.
    #[inline]
    pub fn validate(self, byte: u8) -> Result<(), Expectation<Self>> {
        let detected = Marker::detect(byte);

        if detected != self {
            return Err(Expectation {
                unexpected: detected,
                expected: self,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use test_log::test;

    use super::*;

    const MARKERS: [Marker; 9] = [
        Marker::Null,
        Marker::Unit,
        Marker::Bool,
        Marker::Bytes,
        Marker::Float,
        Marker::Map,
        Marker::Seq,
        Marker::String,
        Marker::Int,
    ];

    fn bytes_for_marker(marker: Marker) -> RangeInclusive<u8> {
        // A byte with only the repr bit set:
        let min_byte = marker as u8;
        // A byte with all bits lower than the repr bit set, too:
        let max_byte = min_byte | min_byte.saturating_sub(1);
        min_byte..=max_byte
    }

    #[test]
    fn detect() {
        for expected in MARKERS {
            for byte in bytes_for_marker(expected) {
                let actual = Marker::detect(byte);
                assert_eq!(actual, expected);
            }
        }
    }

    #[test]
    fn validate() {
        for expected in MARKERS {
            for byte in bytes_for_marker(expected) {
                expected.validate(byte).unwrap();
            }
        }
    }
}
