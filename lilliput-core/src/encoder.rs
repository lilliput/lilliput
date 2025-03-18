use crate::{error::Result, io::Write, value::Value, Profile};

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
    #[allow(dead_code)]
    profile: Profile,
}

impl<W> Encoder<W> {
    pub fn new(writer: W, profile: Profile) -> Self {
        Encoder {
            writer,
            pos: 0,
            profile,
        }
    }
}

impl<W> Encoder<W>
where
    W: Write,
{
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
    fn push_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.writer.write(bytes)?;
        self.pos += bytes.len();

        Ok(())
    }

    #[allow(dead_code)]
    fn existing(&self) -> usize {
        self.pos
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
        let mut encoder = Encoder::new(writer, Profile::None);

        encoder.push_bytes(&[]).unwrap();
        encoder.push_bytes(&[1]).unwrap();
        encoder.push_bytes(&[2, 3]).unwrap();

        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn existing() {
        let mut vec: Vec<u8> = Vec::new();
        let writer = StdIoWriter::new(&mut vec);
        let mut encoder = Encoder::new(writer, Profile::None);
        assert_eq!(encoder.existing(), 0);

        encoder.push_bytes(&[42]).unwrap();
        assert_eq!(encoder.existing(), 1);
    }

    #[test]
    fn into_vec() {
        let mut vec: Vec<u8> = Vec::new();
        let writer = StdIoWriter::new(&mut vec);
        let mut encoder = Encoder::new(writer, Profile::None);
        encoder.push_bytes(&[1, 2, 3]).unwrap();

        assert_eq!(vec, vec![1, 2, 3]);
    }
}
