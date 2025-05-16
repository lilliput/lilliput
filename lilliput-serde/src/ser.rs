use lilliput_core::{
    config::EncodingConfig,
    encoder::Encoder,
    io::{StdIoWriter, Write},
};
use serde::{ser, Serialize};

use crate::{Error, Result};

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub enum StructRepr {
    #[default]
    Seq,
    Map,
}

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub enum EnumVariantRepr {
    #[default]
    Index,
    Name,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct SerializerConfig {
    pub struct_repr: StructRepr,
    pub enum_variant_repr: EnumVariantRepr,
    pub encoder: EncoderConfig,
}

pub struct Serializer<W> {
    pub(crate) encoder: Encoder<W>,
    pub(crate) config: SerializerConfig,
}

impl<W> Serializer<W> {
    pub fn from_writer(writer: W, config: SerializerConfig) -> Self {
        let encoder = Encoder::new(writer, config.encoder.clone());
        Self { encoder, config }
    }
}

pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut vec: Vec<u8> = Vec::new();
    let writer = StdIoWriter::new(&mut vec);
    let config = SerializerConfig::default();
    let mut serializer = Serializer::from_writer(writer, config);

    value.serialize(&mut serializer)?;

    Ok(vec)
}

#[cfg(feature = "std")]
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: ?Sized + Serialize,
{
    let config = SerializerConfig::default();
    let mut serializer = Serializer::from_writer(StdIoWriter::new(writer), config);

    value.serialize(&mut serializer)
}

impl<W> ser::Serializer for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, value: bool) -> Result<()> {
        self.encoder.encode_bool(value)
    }

    fn serialize_i8(self, value: i8) -> Result<()> {
        self.encoder.encode_i64(value.into())
    }

    fn serialize_i16(self, value: i16) -> Result<()> {
        self.encoder.encode_i64(value.into())
    }

    fn serialize_i32(self, value: i32) -> Result<()> {
        self.encoder.encode_i64(value.into())
    }

    fn serialize_i64(self, value: i64) -> Result<()> {
        self.encoder.encode_i64(value)
    }

    fn serialize_u8(self, value: u8) -> Result<()> {
        self.encoder.encode_u64(value.into())
    }

    fn serialize_u16(self, value: u16) -> Result<()> {
        self.encoder.encode_u64(value.into())
    }

    fn serialize_u32(self, value: u32) -> Result<()> {
        self.encoder.encode_u64(value.into())
    }

    fn serialize_u64(self, value: u64) -> Result<()> {
        self.encoder.encode_u64(value)
    }

    fn serialize_f32(self, value: f32) -> Result<()> {
        self.encoder.encode_f32(value)
    }

    fn serialize_f64(self, value: f64) -> Result<()> {
        self.encoder.encode_f64(value)
    }

    fn serialize_char(self, value: char) -> Result<()> {
        self.serialize_str(&value.to_string())
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        self.encoder.encode_str(value)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        self.encoder.encode_bytes(value)
    }

    fn serialize_none(self) -> Result<()> {
        self.encoder.encode_null()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.encoder.encode_unit()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        match self.config.enum_variant_repr {
            EnumVariantRepr::Index => self.serialize_u32(variant_index),
            EnumVariantRepr::Name => self.serialize_str(variant),
        }
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let header = self.encoder.header_for_map_len(1);
        self.encoder.encode_map_header(&header)?;

        match self.config.enum_variant_repr {
            EnumVariantRepr::Index => self.serialize_u32(variant_index)?,
            EnumVariantRepr::Name => self.serialize_str(variant)?,
        }

        value.serialize(&mut *self)?;

        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        let Some(len) = len else {
            return Err(Error::unknown_length());
        };

        let header = self.encoder.header_for_seq_len(len);
        self.encoder.encode_seq_header(&header)?;

        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        let outer_map_header = self.encoder.header_for_map_len(1);
        self.encoder.encode_map_header(&outer_map_header)?;

        match self.config.enum_variant_repr {
            EnumVariantRepr::Index => {
                self.serialize_u32(variant_index)?
            }
            EnumVariantRepr::Name => {
                self.serialize_str(variant)?
            }
        }

        let inner_seq_header = self.encoder.header_for_seq_len(len);
        self.encoder.encode_seq_header(&inner_seq_header)?;

        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        let Some(len) = len else {
            return Err(Error::unknown_length());
        };

        let header = self.encoder.header_for_map_len(len);
        self.encoder.encode_map_header(&header)?;

        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        let outer_map_header = self.encoder.header_for_map_len(1);
        self.encoder.encode_map_header(&outer_map_header)?;

        match self.config.enum_variant_repr {
            EnumVariantRepr::Index => {
                self.serialize_u32(variant_index)?
            }
            EnumVariantRepr::Name => {
                self.serialize_str(variant)?
            }
        }

        let inner_map_header = self.encoder.header_for_map_len(len);
        self.encoder.encode_map_header(&inner_map_header)?;

        Ok(self)
    }
}

impl<W> ser::SerializeSeq for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<W> ser::SerializeTuple for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<W> ser::SerializeTupleStruct for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<W> ser::SerializeTupleVariant for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<W> ser::SerializeMap for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<W> ser::SerializeStruct for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<W> ser::SerializeStructVariant for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}
