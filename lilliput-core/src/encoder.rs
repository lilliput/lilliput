//! Encoders for encoding lilliput values.

use crate::{config::EncoderConfig, error::Result, header::Header, io::Write, value::Value};

mod bool;
mod bytes;
mod float;
mod int;
mod map;
mod null;
mod seq;
mod string;
mod unit;

/// An encoder for encoding lilliput values.
#[derive(Debug)]
pub struct Encoder<W> {
    writer: W,
    pos: usize,
    config: EncoderConfig,
}

impl<W> Encoder<W> {
    /// Creates a encoder from `writer`.
    pub fn new(writer: W) -> Self {
        Self::new_with_config(writer, EncoderConfig::default())
    }

    /// Creates a encoder from `writer`, configured by `config`.
    pub fn new_with_config(writer: W, config: EncoderConfig) -> Self {
        Encoder {
            writer,
            pos: 0,
            config,
        }
    }

    /// Returns the encoder's internal `writer`, consuming `self`.
    pub fn into_writer(self) -> W {
        self.writer
    }

    /// Returns the encoder's current write position.
    pub fn pos(&self) -> usize {
        self.pos
    }
}

impl<W> Encoder<W>
where
    W: Write,
{
    /// Encodes a value's `Header`.
    pub fn encode_header(&mut self, header: &Header) -> Result<()> {
        match header {
            Header::Int(value) => self.encode_int_header(value),
            Header::String(value) => self.encode_string_header(value),
            Header::Seq(value) => self.encode_seq_header(value),
            Header::Map(value) => self.encode_map_header(value),
            Header::Float(value) => self.encode_float_header(value),
            Header::Bytes(value) => self.encode_bytes_header(value),
            Header::Bool(value) => self.encode_bool_header(value),
            Header::Unit(value) => self.encode_unit_header(value),
            Header::Null(value) => self.encode_null_header(value),
        }
    }

    /// Encodes a `Value`.
    pub fn encode_value(&mut self, value: &Value) -> Result<()> {
        match value {
            Value::Int(value) => self.encode_int_value(value),
            Value::String(value) => self.encode_string_value(value),
            Value::Seq(value) => self.encode_seq_value(value),
            Value::Map(value) => self.encode_map_value(value),
            Value::Float(value) => self.encode_float_value(value),
            Value::Bytes(value) => self.encode_bytes_value(value),
            Value::Bool(value) => self.encode_bool_value(value),
            Value::Unit(value) => self.encode_unit_value(value),
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
        let mut encoder = Encoder::new(writer);

        encoder.push_bytes(&[]).unwrap();
        encoder.push_bytes(&[1]).unwrap();
        encoder.push_bytes(&[2, 3]).unwrap();

        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn into_vec() {
        let mut vec: Vec<u8> = Vec::new();
        let writer = StdIoWriter::new(&mut vec);
        let mut encoder = Encoder::new(writer);
        encoder.push_bytes(&[1, 2, 3]).unwrap();

        assert_eq!(vec, vec![1, 2, 3]);
    }
}
