use crate::{
    io::Write,
    value::{
        BoolValue, BytesValue, FloatValue, IntValue, Map, MapValue, NullValue, SeqValue,
        StringValue, Value,
    },
    Profile,
};

mod bool;
mod bytes;
mod float;
mod int;
mod map;
mod null;
mod seq;
mod string;

use self::{bool::*, bytes::*, float::*, int::*, map::*, null::*, seq::*, string::*};

#[derive(Eq, PartialEq, Debug, thiserror::Error)]
pub enum EncoderError {
    #[error("invalid seq")]
    Seq,
    #[error("invalid map")]
    Map,
    #[error("writer error: {0}")]
    Writer(String),
}

#[derive(Debug)]
enum EncoderState {
    Seq { pos: usize, len: usize },
    Map { pos: usize, len: usize },
}

impl EncoderState {
    fn seq(len: usize) -> Self {
        Self::Seq { pos: 0, len }
    }

    fn map(len: usize) -> Self {
        Self::Map {
            pos: 0,
            len: 2 * len,
        }
    }

    fn on_encode_value(&mut self) -> Result<(), EncoderError> {
        match self {
            EncoderState::Seq { pos, len } => {
                if pos < len {
                    *pos += 1;
                    Ok(())
                } else {
                    Err(EncoderError::Seq)
                }
            }
            EncoderState::Map { pos, len } => {
                if pos < len {
                    *pos += 1;
                    Ok(())
                } else {
                    Err(EncoderError::Map)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Encoder<W> {
    writer: W,
    pos: usize,
    #[allow(dead_code)]
    profile: Profile,
    state: Vec<EncoderState>,
}

impl<W> Encoder<W> {
    pub fn new(writer: W, profile: Profile) -> Self {
        Encoder {
            writer,
            pos: 0,
            profile,
            state: vec![],
        }
    }
}

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn into_writer(self) -> Result<W, EncoderError> {
        if let Some(state) = self.state.last() {
            match state {
                EncoderState::Seq { .. } => Err(EncoderError::Seq),
                EncoderState::Map { .. } => Err(EncoderError::Map),
            }
        } else {
            Ok(self.writer)
        }
    }

    // MARK: - Any Values

    pub fn encode_any(&mut self, value: &Value) -> Result<(), EncoderError> {
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

    // MARK: - Int Values

    pub fn encode_i8(&mut self, value: i8) -> Result<(), EncoderError> {
        IntEncoder::with(self).encode_signed(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_i16(&mut self, value: i16) -> Result<(), EncoderError> {
        IntEncoder::with(self).encode_signed(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_i32(&mut self, value: i32) -> Result<(), EncoderError> {
        IntEncoder::with(self).encode_signed(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_i64(&mut self, value: i64) -> Result<(), EncoderError> {
        IntEncoder::with(self).encode_signed(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_u8(&mut self, value: u8) -> Result<(), EncoderError> {
        IntEncoder::with(self).encode_unsigned(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_u16(&mut self, value: u16) -> Result<(), EncoderError> {
        IntEncoder::with(self).encode_unsigned(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_u32(&mut self, value: u32) -> Result<(), EncoderError> {
        IntEncoder::with(self).encode_unsigned(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_u64(&mut self, value: u64) -> Result<(), EncoderError> {
        IntEncoder::with(self).encode_unsigned(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_int_value(&mut self, value: &IntValue) -> Result<(), EncoderError> {
        IntEncoder::with(self).encode_int_value(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    // MARK: - String Values

    pub fn encode_string(&mut self, value: &str) -> Result<(), EncoderError> {
        StringEncoder::with(self).encode_string(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub(crate) fn encode_string_value(&mut self, value: &StringValue) -> Result<(), EncoderError> {
        StringEncoder::with(self).encode_string_value(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    // MARK: - Seq Values

    pub fn encode_seq(&mut self, value: &[Value]) -> Result<(), EncoderError> {
        SeqEncoder::with(self).encode_seq(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub(crate) fn encode_seq_value(&mut self, value: &SeqValue) -> Result<(), EncoderError> {
        SeqEncoder::with(self).encode_seq_value(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_seq_start(&mut self, len: usize) -> Result<(), EncoderError> {
        SeqEncoder::with(self).encode_seq_start(len)?;

        self.state.push(EncoderState::seq(len));

        Ok(())
    }

    pub fn encode_seq_end(&mut self) -> Result<(), EncoderError> {
        let Some(state) = self.state.last() else {
            return Err(EncoderError::Seq);
        };

        let EncoderState::Seq { pos, len } = state else {
            return Err(EncoderError::Seq);
        };

        if pos != len {
            return Err(EncoderError::Seq);
        }

        let _ = self.state.pop();

        SeqEncoder::with(self).encode_seq_end()?;

        self.on_encode_value()?;

        Ok(())
    }

    // MARK: - Map Values

    pub fn encode_map(&mut self, value: &Map) -> Result<(), EncoderError> {
        MapEncoder::with(self).encode_map(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub(crate) fn encode_map_value(&mut self, value: &MapValue) -> Result<(), EncoderError> {
        MapEncoder::with(self).encode_map_value(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_map_start(&mut self, len: usize) -> Result<(), EncoderError> {
        MapEncoder::with(self).encode_map_start(len)?;

        self.state.push(EncoderState::map(len));

        Ok(())
    }

    pub fn encode_map_end(&mut self) -> Result<(), EncoderError> {
        let Some(state) = self.state.last() else {
            return Err(EncoderError::Map);
        };

        let EncoderState::Map { pos, len } = state else {
            return Err(EncoderError::Map);
        };

        if pos != len {
            return Err(EncoderError::Map);
        }

        let _ = self.state.pop();

        MapEncoder::with(self).encode_map_end()?;

        self.on_encode_value()?;

        Ok(())
    }

    // MARK: - Float Values

    pub fn encode_f32(&mut self, value: f32) -> Result<(), EncoderError> {
        FloatEncoder::with(self).encode_float(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_f64(&mut self, value: f64) -> Result<(), EncoderError> {
        FloatEncoder::with(self).encode_float(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_float_value(&mut self, value: &FloatValue) -> Result<(), EncoderError> {
        FloatEncoder::with(self).encode_float_value(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    // MARK: - Bytes Values

    pub fn encode_bytes(&mut self, value: &[u8]) -> Result<(), EncoderError> {
        BytesEncoder::with(self).encode_bytes(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_bytes_value(&mut self, value: &BytesValue) -> Result<(), EncoderError> {
        BytesEncoder::with(self).encode_bytes_value(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    // MARK: - Bool Values

    pub fn encode_bool(&mut self, value: bool) -> Result<(), EncoderError> {
        BoolEncoder::with(self).encode_bool(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_bool_value(&mut self, value: &BoolValue) -> Result<(), EncoderError> {
        BoolEncoder::with(self).encode_bool_value(value)?;

        self.on_encode_value()?;

        Ok(())
    }

    // MARK: - Null Values

    pub fn encode_null(&mut self) -> Result<(), EncoderError> {
        NullEncoder::with(self).encode_null()?;

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_null_value(&mut self, value: &NullValue) -> Result<(), EncoderError> {
        NullEncoder::with(self).encode_null_value(value)?;

        self.on_encode_value()?;

        Ok(())
    }
}

// MARK: - Auxiliary Methods

impl<W> Encoder<W>
where
    W: Write,
{
    fn push_bytes(&mut self, bytes: &[u8]) -> Result<(), EncoderError> {
        self.writer
            .write(bytes)
            .map_err(|err| EncoderError::Writer(err.to_string()))?;
        self.pos += bytes.len();

        Ok(())
    }

    #[allow(dead_code)]
    fn existing(&self) -> usize {
        self.pos
    }

    fn on_encode_value(&mut self) -> Result<(), EncoderError> {
        if let Some(state) = self.state.last_mut() {
            state.on_encode_value()
        } else {
            Ok(())
        }
    }
}

// MARK: - Tests

#[cfg(test)]
mod test {
    use crate::io::StdIoWriter;

    use super::*;

    #[test]
    fn push_bytes() {
        let buffer: StdIoWriter<Vec<u8>> = StdIoWriter(vec![]);
        let mut encoder = Encoder::new(buffer, Profile::None);

        encoder.push_bytes(&[1]).unwrap();
        assert_eq!(encoder.writer.0, vec![1]);

        encoder.push_bytes(&[2, 3]).unwrap();
        assert_eq!(encoder.writer.0, vec![1, 2, 3]);
    }

    #[test]
    fn existing() {
        let buffer: StdIoWriter<Vec<u8>> = StdIoWriter(vec![]);
        let mut encoder = Encoder::new(buffer, Profile::None);
        assert_eq!(encoder.existing(), 0);

        encoder.push_bytes(&[42]).unwrap();
        assert_eq!(encoder.existing(), 1);
    }

    #[test]
    fn into_vec() {
        let buffer: StdIoWriter<Vec<u8>> = StdIoWriter(vec![]);
        let mut encoder = Encoder::new(buffer, Profile::None);
        encoder.push_bytes(&[1, 2, 3]).unwrap();

        assert_eq!(encoder.into_writer().unwrap().0, vec![1, 2, 3]);
    }
}
