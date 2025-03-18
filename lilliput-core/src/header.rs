mod bool;
mod bytes;
mod float;
mod int;
mod map;
mod null;
mod seq;
mod string;

use crate::error::Expectation;

pub use self::{
    bool::BoolHeader, bytes::BytesHeader, float::FloatHeader, int::IntHeader, map::MapHeader,
    null::NullHeader, seq::SeqHeader, string::StringHeader,
};

pub trait DecodeHeader: Sized {
    fn decode(byte: u8) -> Result<Self, Expectation<Marker>>;
}

pub trait EncodeHeader: Sized {
    fn encode(self) -> u8;
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
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
    pub fn of(header: &Header) -> Self {
        match header {
            Header::Int(_) => Marker::Int,
            Header::String(_) => Marker::String,
            Header::Seq(_) => Marker::Seq,
            Header::Map(_) => Marker::Map,
            Header::Float(_) => Marker::Float,
            Header::Bytes(_) => Marker::Bytes,
            Header::Bool(_) => Marker::Bool,
            Header::Null(_) => Marker::Null,
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

    fn validate(self, byte: u8) -> Result<(), Expectation<Self>> {
        let detected = Marker::detect(byte);
        let is_valid = detected == self;

        // let type_bit = self as u8;
        // let mask = !type_bit.saturating_sub(1);
        // let masked_byte = byte & mask;
        // let is_valid = masked_byte == type_bit;

        if !is_valid {
            // let detected = Marker::detect(byte);
            return Err(Expectation {
                unexpected: detected,
                expected: self,
            });
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

impl DecodeHeader for Header {
    fn decode(byte: u8) -> Result<Self, Expectation<Marker>> {
        match Marker::detect(byte) {
            Marker::Int => Ok(Header::Int(IntHeader::decode(byte)?)),
            Marker::String => Ok(Header::String(StringHeader::decode(byte)?)),
            Marker::Seq => Ok(Header::Seq(SeqHeader::decode(byte)?)),
            Marker::Map => Ok(Header::Map(MapHeader::decode(byte)?)),
            Marker::Float => Ok(Header::Float(FloatHeader::decode(byte)?)),
            Marker::Bytes => Ok(Header::Bytes(BytesHeader::decode(byte)?)),
            Marker::Bool => Ok(Header::Bool(BoolHeader::decode(byte)?)),
            Marker::Null => Ok(Header::Null(NullHeader::decode(byte)?)),
            Marker::Reserved => unimplemented!(),
        }
    }
}
