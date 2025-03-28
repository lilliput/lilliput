mod bool;
mod bytes;
mod float;
mod int;
mod map;
mod null;
mod seq;
mod string;

use crate::{error::Expectation, marker::Marker};

pub use self::{
    bool::BoolHeader,
    bytes::BytesHeader,
    float::FloatHeader,
    int::{IntHeader, IntHeaderRepr},
    map::{MapHeader, MapHeaderRepr},
    null::NullHeader,
    seq::{SeqHeader, SeqHeaderRepr},
    string::{StringHeader, StringHeaderRepr},
};

pub trait DecodeHeader: Sized {
    fn decode(byte: u8) -> Result<Self, Expectation<Marker>>;
}

pub trait EncodeHeader: Sized {
    fn encode(self) -> u8;
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
    #[inline]
    fn default() -> Self {
        Self::Null(NullHeader)
    }
}

impl From<IntHeader> for Header {
    #[inline]
    fn from(value: IntHeader) -> Self {
        Self::Int(value)
    }
}

impl From<StringHeader> for Header {
    #[inline]
    fn from(value: StringHeader) -> Self {
        Self::String(value)
    }
}

impl From<SeqHeader> for Header {
    #[inline]
    fn from(value: SeqHeader) -> Self {
        Self::Seq(value)
    }
}

impl From<MapHeader> for Header {
    #[inline]
    fn from(value: MapHeader) -> Self {
        Self::Map(value)
    }
}

impl From<FloatHeader> for Header {
    #[inline]
    fn from(value: FloatHeader) -> Self {
        Self::Float(value)
    }
}

impl From<BytesHeader> for Header {
    #[inline]
    fn from(value: BytesHeader) -> Self {
        Self::Bytes(value)
    }
}

impl From<BoolHeader> for Header {
    #[inline]
    fn from(value: BoolHeader) -> Self {
        Self::Bool(value)
    }
}

impl From<NullHeader> for Header {
    #[inline]
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

impl Header {
    pub fn marker(&self) -> Marker {
        match self {
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
}
