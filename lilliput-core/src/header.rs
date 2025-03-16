mod bool;
mod bytes;
mod float;
mod int;
mod map;
mod null;
mod seq;
mod string;

pub use self::{
    bool::BoolHeader, bytes::BytesHeader, float::FloatHeader, int::IntHeader, map::MapHeader,
    null::NullHeader, seq::SeqHeader, string::StringHeader,
};

#[derive(Eq, PartialEq, Debug, thiserror::Error)]
pub enum HeaderDecodeError {
    #[error("expected type {expected:?}, found {actual:?}")]
    UnexpectedType {
        expected: HeaderType,
        actual: HeaderType,
    },
}

pub trait DecodeHeader: Sized {
    fn decode(byte: u8) -> Result<Self, HeaderDecodeError>;
}

pub trait EncodeHeader: Sized {
    fn encode(self) -> u8;
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum HeaderType {
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

impl HeaderType {
    pub fn of(header: &Header) -> Self {
        match header {
            Header::Int(_) => HeaderType::Int,
            Header::String(_) => HeaderType::String,
            Header::Seq(_) => HeaderType::Seq,
            Header::Map(_) => HeaderType::Map,
            Header::Float(_) => HeaderType::Float,
            Header::Bytes(_) => HeaderType::Bytes,
            Header::Bool(_) => HeaderType::Bool,
            Header::Null(_) => HeaderType::Null,
        }
    }

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

    fn validate(self, byte: u8) -> Result<(), HeaderDecodeError> {
        let type_bit = self as u8;
        let mask = !type_bit.saturating_sub(1);
        let masked_byte = byte & mask;

        if masked_byte != type_bit {
            return Err(HeaderDecodeError::UnexpectedType {
                expected: HeaderType::Map,
                actual: HeaderType::detect(byte),
            });
        }

        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Header {
    /// Represents a integer number.
    Int(IntHeader),

    /// Represents a string.
    String(StringHeader),

    /// Represents a sequence of values.
    Seq(SeqHeader),

    /// Represents a map of key-value pairs.
    ///
    /// By default the map is backed by a `BTreeMap`. Enable the `preserve_order`
    /// feature of serde_lilliput to use `OrderMap` instead, which preserves
    /// entries in the order they are inserted into the map.
    Map(MapHeader),

    /// Represents a floating-point number.
    Float(FloatHeader),

    /// Represents a byte array.
    Bytes(BytesHeader),

    /// Represents a boolean.
    Bool(BoolHeader),

    /// Represents a null value.
    Null(NullHeader),
}

impl Default for Header {
    fn default() -> Self {
        Self::Null(NullHeader)
    }
}

impl From<IntHeader> for Header {
    fn from(value: IntHeader) -> Self {
        Self::Int(value)
    }
}

impl From<StringHeader> for Header {
    fn from(value: StringHeader) -> Self {
        Self::String(value)
    }
}

impl From<SeqHeader> for Header {
    fn from(value: SeqHeader) -> Self {
        Self::Seq(value)
    }
}

impl From<MapHeader> for Header {
    fn from(value: MapHeader) -> Self {
        Self::Map(value)
    }
}

impl From<FloatHeader> for Header {
    fn from(value: FloatHeader) -> Self {
        Self::Float(value)
    }
}

impl From<BytesHeader> for Header {
    fn from(value: BytesHeader) -> Self {
        Self::Bytes(value)
    }
}

impl From<BoolHeader> for Header {
    fn from(value: BoolHeader) -> Self {
        Self::Bool(value)
    }
}

impl From<NullHeader> for Header {
    fn from(value: NullHeader) -> Self {
        Self::Null(value)
    }
}

impl Header {
    pub fn has_type(&self, header_type: HeaderType) -> bool {
        HeaderType::of(self) == header_type
    }
}
