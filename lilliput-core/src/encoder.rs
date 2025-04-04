use num_traits::{Signed, Unsigned};

use crate::{
    config::EncodingConfig,
    error::Result,
    header::{IntHeader, MapHeader, SeqHeader, StringHeader},
    io::Write,
    num::WithPackedBeBytes,
    value::{IntValue, Value},
};

mod bool;
mod bytes;
mod float;
mod int;
mod map;
mod null;
mod seq;
mod string;

#[derive(Debug)]
pub struct Encoder<W> {
    writer: W,
    pos: usize,
    config: EncodingConfig,
}

impl<W> Encoder<W> {
    pub fn new(writer: W, config: EncodingConfig) -> Self {
        Encoder {
            writer,
            pos: 0,
            config,
        }
    }
}

impl<W> Encoder<W>
where
    W: Write,
{
    // pub fn header_for_int_value(&self, value: IntValue) -> IntHeader {
    //     IntHeader::new(value, self.config.int_packing)
    // }

    pub fn header_for_signed_int<T>(&self, value: T) -> IntHeader
    where
        T: Signed + WithPackedBeBytes,
    {
        IntHeader::signed(value, self.config.int_packing)
    }

    pub fn header_for_unsigned_int<T>(&self, value: T) -> IntHeader
    where
        T: Unsigned + WithPackedBeBytes,
    {
        IntHeader::unsigned(value, self.config.int_packing)
    }

    pub fn header_for_map(&self, len: usize) -> MapHeader {
        MapHeader::new(len, self.config.len_packing)
    }

    pub fn header_for_seq(&self, len: usize) -> SeqHeader {
        SeqHeader::new(len, self.config.len_packing)
    }

    pub fn header_for_string(&self, len: usize) -> StringHeader {
        StringHeader::new(len, self.config.len_packing)
    }

    pub fn encode_any(&mut self, value: &Value) -> Result<()> {
        match value {
            Value::Int(value) => self.encode_int_value(value),
            Value::String(value) => self.encode_string_value(value),
            Value::Seq(value) => self.encode_seq_value(value),
            Value::Map(value) => self.encode_map_value(value),
            Value::Float(value) => self.encode_float_value(value),
            Value::Bytes(value) => self.encode_bytes_value(value),
            Value::Bool(value) => self.encode_bool_value(value),
            Value::Null(value) => self.encode_null_value(value),
        }
    }
}

// MARK: - Auxiliary Methods

impl<W> Encoder<W>
where
    W: Write,
{
    #[inline]
    fn push_byte(&mut self, byte: u8) -> Result<()> {
        self.push_bytes(&[byte])
    }

    fn push_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.writer.write(bytes)?;
        self.pos += bytes.len();

        Ok(())
    }
}

// MARK: - Tests

#[cfg(test)]
mod test {
    use crate::io::{StdIoWriter, VecWriter};

    use super::*;

    #[test]
    fn push_bytes() {
        let mut vec: Vec<u8> = Vec::new();
        let writer = VecWriter::new(&mut vec);
        let mut encoder = Encoder::new(writer, EncodingConfig::default());

        encoder.push_bytes(&[]).unwrap();
        encoder.push_bytes(&[1]).unwrap();
        encoder.push_bytes(&[2, 3]).unwrap();

        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn into_vec() {
        let mut vec: Vec<u8> = Vec::new();
        let writer = StdIoWriter::new(&mut vec);
        let mut encoder = Encoder::new(writer, EncodingConfig::default());
        encoder.push_bytes(&[1, 2, 3]).unwrap();

        assert_eq!(vec, vec![1, 2, 3]);
    }
}
