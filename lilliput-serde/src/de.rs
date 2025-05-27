use serde::{
    de::{self, Error as _, IntoDeserializer as _},
    Deserialize, Deserializer as _,
};

use lilliput_core::{
    decoder::Decoder,
    io::{Read, Reference, SliceReader, StdIoReader},
    marker::Marker,
    value::{FloatValue, IntValue, SignedIntValue, UnsignedIntValue},
};

use crate::error::{Error, Result};

pub struct Deserializer<R> {
    decoder: Decoder<R>,
    scratch: Vec<u8>,
    remaining_depth: u8,
    #[cfg(feature = "unbounded_depth")]
    disable_depth_limit: bool,
}

impl<R> Deserializer<R> {
    pub fn from_reader(reader: R) -> Self {
        Deserializer {
            decoder: Decoder::new(reader),
            scratch: Vec::new(),
            remaining_depth: 128,
            #[cfg(feature = "unbounded_depth")]
            disable_depth_limit: false,
        }
    }

    /// Parse arbitrarily deep Lilliput structures without any consideration for
    /// overflowing the stack.
    ///
    /// You will want to provide some other way to protect against stack
    /// overflows, such as by wrapping your Deserializer in the dynamically
    /// growing stack adapter provided by the serde_stacker crate. Additionally
    /// you will need to be careful around other recursive operations on the
    /// parsed result which may overflow the stack after deserialization has
    /// completed, including, but not limited to, Display and Debug and Drop
    /// impls.
    ///
    /// *This method is only available if lilliput_serde is built with the
    /// `"unbounded_depth"` feature.*
    #[cfg(feature = "unbounded_depth")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unbounded_depth")))]
    pub fn disable_depth_limit(&mut self) {
        self.disable_depth_limit = true;
    }
}

pub fn from_slice<'de, T>(bytes: &'de [u8]) -> Result<T>
where
    T: 'de + Deserialize<'de>,
{
    let reader = SliceReader::new(bytes);
    T::deserialize(&mut Deserializer::from_reader(reader))
}

#[cfg(feature = "std")]
pub fn from_reader<R, T>(reader: R) -> Result<T>
where
    R: std::io::Read,
    T: de::DeserializeOwned,
{
    let reader = StdIoReader::new(reader);
    T::deserialize(&mut Deserializer::from_reader(reader))
}

#[cfg(not(feature = "unbounded_depth"))]
macro_rules! if_checking_depth_limit {
    (this: $this:ident; $($body:tt)*) => {
        $($body)*
    };
}

#[cfg(feature = "unbounded_depth")]
macro_rules! if_checking_depth_limit {
    (this: $this:ident; $($body:tt)*) => {
        if !$this.disable_depth_limit {
            $($body)*
        }
    };
}

macro_rules! check_depth {
    (this: $this:ident; $($body:tt)*) => {
        if_checking_depth_limit! {
            this: $this;

            $this.remaining_depth -= 1;
            if $this.remaining_depth == 0 {
                return Err(Error::depth_limit_exceeded(Some($this.decoder.pos())));
            }
        }

        $($body)*

        if_checking_depth_limit! {
            this: $this;

            $this.remaining_depth += 1;
        }
    };
}

impl<'de, 'a, R> de::Deserializer<'de> for &'a mut Deserializer<R>
where
    R: Read<'de> + 'a,
{
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.decoder.peek_marker()? {
            Marker::Int => self.deserialize_int(visitor),
            Marker::String => self.deserialize_str(visitor),
            Marker::Seq => self.deserialize_seq(visitor),
            Marker::Map => self.deserialize_map(visitor),
            Marker::Float => self.deserialize_float(visitor),
            Marker::Bytes => self.deserialize_bytes(visitor),
            Marker::Bool => self.deserialize_bool(visitor),
            Marker::Unit => self.deserialize_unit(visitor),
            Marker::Null => self.deserialize_option(visitor),
        }
    }

    #[inline]
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bool(self.decoder.decode_bool()?)
    }

    #[inline]
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.decoder.decode_i8()?)
    }

    #[inline]
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.decoder.decode_i16()?)
    }

    #[inline]
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.decoder.decode_i32()?)
    }

    #[inline]
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.decoder.decode_i64()?)
    }

    #[inline]
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i128(self.decoder.decode_i64()? as i128)
    }

    #[inline]
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.decoder.decode_u8()?)
    }

    #[inline]
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.decoder.decode_u16()?)
    }

    #[inline]
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.decoder.decode_u32()?)
    }

    #[inline]
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.decoder.decode_u64()?)
    }

    #[inline]
    fn deserialize_u128<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u128(self.decoder.decode_u64()? as u128)
    }

    #[inline]
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(self.decoder.decode_f32()?)
    }

    #[inline]
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(self.decoder.decode_f64()?)
    }

    #[inline]
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.decoder.decode_str(&mut self.scratch)? {
            Reference::Borrowed(str) => visitor.visit_borrowed_str(str),
            Reference::Copied(str) => visitor.visit_str(str),
        }
    }

    #[inline]
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(self.decoder.decode_string()?)
    }

    #[inline]
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.decoder.peek_marker()? == Marker::Seq {
            let header = self.decoder.decode_seq_header()?;
            let mut bytes: Vec<u8> = Vec::new();
            for _ in 0..header.len() {
                bytes.push(self.decoder.decode_u8()?);
            }
            visitor.visit_bytes(&bytes)
        } else {
            match self.decoder.decode_bytes(&mut self.scratch)? {
                Reference::Borrowed(bytes) => visitor.visit_borrowed_bytes(bytes),
                Reference::Copied(bytes) => visitor.visit_bytes(bytes),
            }
        }
    }

    #[inline]
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.decoder.decode_bytes_buf()?)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.decoder.peek_marker()? == Marker::Null {
            true => {
                self.decoder.decode_null()?;
                visitor.visit_none()
            }
            false => visitor.visit_some(self),
        }
    }

    #[inline]
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.decoder.decode_unit()?;
        visitor.visit_unit()
    }

    #[inline]
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    #[inline]
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let header = self.decoder.decode_seq_header()?;

        check_depth! {
            this: self;
            let value = visitor.visit_seq(SeqAccess::new(self, header.len()))?;
        }

        Ok(value)
    }

    #[inline]
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    #[inline]
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    #[inline]
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let header = self.decoder.decode_map_header()?;

        check_depth! {
            this: self;
            let value = visitor.visit_map(MapAccess::new(self, header.len()))?;
        }

        Ok(value)
    }

    #[inline]
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.decoder.peek_marker()? {
            Marker::Int => {
                let index = self.decoder.decode_u32()? as usize;
                visitor.visit_enum(variants[index].into_deserializer())
            }
            Marker::String => {
                let mut scratch = vec![];
                let str_ref = self.decoder.decode_str(&mut scratch)?;
                visitor.visit_enum(str_ref.into_deserializer())
            }
            Marker::Map => {
                let header = self.decoder.decode_map_header()?;

                if header.len() != 1 {
                    return Err(Error::custom("expected map of length 1"));
                }

                check_depth! {
                    this: self;
                    let marker = self.decoder.peek_marker()?;
                    let result = visitor.visit_enum(EnumAccess::new(self, variants, marker));
                }

                result
            }
            other => {
                let pos = self.decoder.pos();
                Err(Error::invalid_type(
                    other.to_string(),
                    "int, string or map".to_owned(),
                    Some(pos),
                ))
            }
        }
    }

    #[inline]
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

impl<'de, R> Deserializer<R>
where
    R: Read<'de>,
{
    #[inline]
    fn pos(&self) -> usize {
        self.decoder.pos()
    }

    #[inline]
    fn deserialize_float<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.decoder.decode_float_value()? {
            FloatValue::F32(value) => visitor.visit_f32(value),
            FloatValue::F64(value) => visitor.visit_f64(value),
        }
    }

    #[inline]
    fn deserialize_int<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.decoder.decode_int_value()? {
            IntValue::Signed(value) => match value {
                SignedIntValue::I8(value) => visitor.visit_i8(value),
                SignedIntValue::I16(value) => visitor.visit_i16(value),
                SignedIntValue::I32(value) => visitor.visit_i32(value),
                SignedIntValue::I64(value) => visitor.visit_i64(value),
            },
            IntValue::Unsigned(value) => match value {
                UnsignedIntValue::U8(value) => visitor.visit_u8(value),
                UnsignedIntValue::U16(value) => visitor.visit_u16(value),
                UnsignedIntValue::U32(value) => visitor.visit_u32(value),
                UnsignedIntValue::U64(value) => visitor.visit_u64(value),
            },
        }
    }
}

struct SeqAccess<'a, R> {
    de: &'a mut Deserializer<R>,
    remaining: usize,
}

impl<'a, R: 'a> SeqAccess<'a, R> {
    #[inline]
    fn new(de: &'a mut Deserializer<R>, count: usize) -> Self {
        SeqAccess {
            de,
            remaining: count,
        }
    }
}

impl<'de, 'a, R> de::SeqAccess<'de> for SeqAccess<'a, R>
where
    R: Read<'de> + 'a,
{
    type Error = Error;

    #[inline]
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.remaining == 0 {
            return Ok(None);
        }

        self.remaining -= 1;

        Ok(Some(seed.deserialize(&mut *self.de)?))
    }
}

struct MapAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
    remaining: usize,
}

impl<'a, R: 'a> MapAccess<'a, R> {
    #[inline]
    fn new(de: &'a mut Deserializer<R>, count: usize) -> Self {
        MapAccess {
            de,
            remaining: count,
        }
    }
}

impl<'de, 'a, R> de::MapAccess<'de> for MapAccess<'a, R>
where
    R: Read<'de> + 'a,
{
    type Error = Error;

    #[inline]
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.remaining == 0 {
            return Ok(None);
        }

        seed.deserialize(&mut *self.de).map(Some)
    }

    #[inline]
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        self.remaining -= 1;

        seed.deserialize(&mut *self.de)
    }
}

struct EnumAccess<'a, R> {
    de: &'a mut Deserializer<R>,
    #[allow(dead_code)]
    variants: &'static [&'static str],
    peeked_marker: Marker,
}

impl<'a, R> EnumAccess<'a, R>
where
    R: 'a,
{
    pub fn new(
        de: &'a mut Deserializer<R>,
        variants: &'static [&'static str],
        peeked_marker: Marker,
    ) -> Self {
        EnumAccess {
            de,
            variants,
            peeked_marker,
        }
    }
}

impl<'de, 'a, R> de::EnumAccess<'de> for EnumAccess<'a, R>
where
    R: Read<'de> + 'a,
{
    type Error = Error;
    type Variant = Self;

    #[inline]
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let value = match self.peeked_marker {
            Marker::Int => {
                let index = u32::deserialize(&mut *self.de)?;
                seed.deserialize(index.into_deserializer())?
            }
            Marker::String => {
                let str = <&str>::deserialize(&mut *self.de)?;
                seed.deserialize(str.into_deserializer())?
            }
            other => {
                return Err(Error::invalid_type(
                    other.to_string(),
                    "int, string".to_owned(),
                    Some(self.de.pos()),
                ))
            }
        };

        Ok((value, self))
    }
}

impl<'de, 'a, R> de::VariantAccess<'de> for EnumAccess<'a, R>
where
    R: Read<'de> + 'a,
{
    type Error = Error;

    #[inline]
    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    #[inline]
    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_tuple(len, visitor)
    }

    #[inline]
    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_map(visitor)
    }
}
