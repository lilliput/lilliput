use crate::error::Expectation;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Marker {
    Int = 0b10000000,
    String = 0b01000000,
    Seq = 0b00100000,
    Map = 0b00010000,
    Float = 0b00001000,
    Bytes = 0b00000100,
    Bool = 0b00000010,
    Null = 0b00000001,
    Reserved = 0b00000000,
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
            Self::Reserved => write!(f, "reserved"),
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
    #[inline]
    fn from_repr(repr: u8) -> Self {
        match repr {
            0b10000000 => Self::Int,
            0b01000000 => Self::String,
            0b00100000 => Self::Seq,
            0b00010000 => Self::Map,
            0b00001000 => Self::Float,
            0b00000100 => Self::Bytes,
            0b00000010 => Self::Bool,
            0b00000001 => Self::Null,
            0b00000000 => Self::Reserved,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn detect(byte: u8) -> Self {
        match byte.leading_zeros() {
            // 0b10000000
            0 => Self::Int,
            // 0b01000000
            1 => Self::String,
            // 0b00100000
            2 => Self::Seq,
            // 0b00010000
            3 => Self::Map,
            // 0b00001000
            4 => Self::Float,
            // 0b00000100
            5 => Self::Bytes,
            // 0b00000010
            6 => Self::Bool,
            // 0b00000001
            7 => Self::Null,
            // 0b00000000
            _ => Self::Reserved,
        }
    }

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

    use super::*;

    const MARKERS: [Marker; 9] = [
        Marker::Reserved,
        Marker::Null,
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
