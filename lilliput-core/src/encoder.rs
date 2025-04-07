use crate::{config::EncodingConfig, error::Result, header::Header, io::Write, value::Value};

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

    pub fn into_writer(self) -> W {
        self.writer
    }
}

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_header(&mut self, header: &Header) -> Result<()> {
        match header {
            Header::Int(value) => self.encode_int_header(value),
            Header::String(value) => self.encode_string_header(value),
            Header::Seq(value) => self.encode_seq_header(value),
            Header::Map(value) => self.encode_map_header(value),
            Header::Float(value) => self.encode_float_header(value),
            Header::Bytes(value) => self.encode_bytes_header(value),
            Header::Bool(value) => self.encode_bool_header(value),
            Header::Null(value) => self.encode_null_header(value),
        }
    }

    pub fn encode_value(&mut self, value: &Value) -> Result<()> {
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
