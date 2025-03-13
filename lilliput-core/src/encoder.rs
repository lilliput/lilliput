use num_traits::{float::FloatCore, PrimInt, Signed, ToBytes, Unsigned};

use crate::{
    num::ToZigZag,
    value::{
        BoolValue, BytesValue, FloatValue, IntValue, Map, MapValue, NullValue, SeqValue,
        SignedIntValue, StringValue, UnsignedIntValue, Value,
    },
    Profile,
};

#[derive(Eq, PartialEq, Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid seq")]
    Seq,
    #[error("invalid map")]
    Map,
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

    fn on_encode_value(&mut self) -> Result<(), Error> {
        match self {
            EncoderState::Seq { pos, len } => {
                if pos < len {
                    *pos += 1;
                    Ok(())
                } else {
                    Err(Error::Seq)
                }
            }
            EncoderState::Map { pos, len } => {
                if pos < len {
                    *pos += 1;
                    Ok(())
                } else {
                    Err(Error::Map)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Encoder {
    buf: Vec<u8>,
    pos: usize,
    #[allow(dead_code)]
    profile: Profile,
    state: Vec<EncoderState>,
}

impl Encoder {
    pub fn new(profile: Profile) -> Self {
        Encoder {
            buf: vec![],
            pos: 0,
            profile,
            state: vec![],
        }
    }
}

impl Encoder {
    pub fn into_vec(self) -> Result<Vec<u8>, Error> {
        if let Some(state) = self.state.last() {
            match state {
                EncoderState::Seq { .. } => Err(Error::Seq),
                EncoderState::Map { .. } => Err(Error::Map),
            }
        } else {
            Ok(self.buf)
        }
    }

    pub fn encode_any(&mut self, value: &Value) -> Result<(), Error> {
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

    pub fn encode_i8(&mut self, value: i8) -> Result<(), Error> {
        self.encode_signed(value)
    }

    pub fn encode_i16(&mut self, value: i16) -> Result<(), Error> {
        self.encode_signed(value)
    }

    pub fn encode_i32(&mut self, value: i32) -> Result<(), Error> {
        self.encode_signed(value)
    }

    pub fn encode_i64(&mut self, value: i64) -> Result<(), Error> {
        self.encode_signed(value)
    }

    pub fn encode_u8(&mut self, value: u8) -> Result<(), Error> {
        self.encode_unsigned(value)
    }

    pub fn encode_u16(&mut self, value: u16) -> Result<(), Error> {
        self.encode_unsigned(value)
    }

    pub fn encode_u32(&mut self, value: u32) -> Result<(), Error> {
        self.encode_unsigned(value)
    }

    pub fn encode_u64(&mut self, value: u64) -> Result<(), Error> {
        self.encode_unsigned(value)
    }

    fn encode_signed<S, U, const N: usize>(&mut self, value: S) -> Result<(), Error>
    where
        S: Signed + ToZigZag<ZigZag = U>,
        U: ToBytes<Bytes = [u8; N]>,
    {
        // Push the value's metadata:
        let mut head_byte = IntValue::PREFIX_BIT;
        head_byte |= IntValue::VARIANT_BIT;
        head_byte |= IntValue::SIGNEDNESS_BIT;

        let unsigned = value.to_zig_zag();
        let bytes = unsigned.to_be_bytes();

        // Push the value's actual bytes:
        head_byte |= (N as u8) - 1; // width of T in bytes, minus 1
        self.push_byte(head_byte)?;

        self.push_bytes(&bytes)?;

        self.on_encode_value()
    }

    fn encode_unsigned<T, const N: usize>(&mut self, value: T) -> Result<(), Error>
    where
        T: Unsigned + PrimInt + ToBytes<Bytes = [u8; N]>,
    {
        // Push the value's metadata:
        let mut head_byte = IntValue::PREFIX_BIT;
        head_byte |= IntValue::VARIANT_BIT;

        let unsigned = value;
        let bytes = unsigned.to_be_bytes();

        // Push the value's actual bytes:
        head_byte |= (N as u8) - 1; // width of T in bytes, minus 1
        self.push_byte(head_byte)?;

        self.push_bytes(&bytes)?;

        self.on_encode_value()
    }

    pub fn encode_int_value(&mut self, value: &IntValue) -> Result<(), Error> {
        match value {
            IntValue::Signed(value) => match *value {
                SignedIntValue::I8(value) => self.encode_i8(value),
                SignedIntValue::I16(value) => self.encode_i16(value),
                SignedIntValue::I32(value) => self.encode_i32(value),
                SignedIntValue::I64(value) => self.encode_i64(value),
            },
            IntValue::Unsigned(value) => match *value {
                UnsignedIntValue::U8(value) => self.encode_u8(value),
                UnsignedIntValue::U16(value) => self.encode_u16(value),
                UnsignedIntValue::U32(value) => self.encode_u32(value),
                UnsignedIntValue::U64(value) => self.encode_u64(value),
            },
        }
    }

    pub fn encode_string(&mut self, value: &str) -> Result<(), Error> {
        let value: &str = value;

        // Push the value's metadata:
        let mut head_byte = StringValue::PREFIX_BIT;
        head_byte |= StringValue::VARIANT_BIT;

        head_byte |= 8 - 1; // width, minus 1
        self.push_byte(head_byte)?;

        // Push the value's length:
        let neck_bytes = value.len().to_be_bytes();
        self.push_bytes(&neck_bytes)?;

        // Push the value's actual bytes:
        let tail_bytes = value.as_bytes();
        self.push_bytes(tail_bytes)?;

        self.on_encode_value()
    }

    pub(crate) fn encode_string_value(&mut self, value: &StringValue) -> Result<(), Error> {
        self.encode_string(&value.0)
    }

    pub fn encode_seq(&mut self, value: &[Value]) -> Result<(), Error> {
        self.encode_seq_start(value.len())?;

        for value in value {
            self.encode_any(value)?;
        }

        self.encode_seq_end()
    }

    pub(crate) fn encode_seq_value(&mut self, value: &SeqValue) -> Result<(), Error> {
        self.encode_seq(&value.0)
    }

    pub fn encode_seq_start(&mut self, len: usize) -> Result<(), Error> {
        // Push the value's metadata:
        let mut head_byte = SeqValue::PREFIX_BIT;
        head_byte |= SeqValue::VARIANT_BIT;
        head_byte |= 8 - 1; // width, minus 1
        self.push_byte(head_byte)?;

        // Push the value's length:
        let neck_bytes = len.to_be_bytes();
        self.push_bytes(&neck_bytes)?;

        self.state.push(EncoderState::seq(len));

        Ok(())
    }

    pub fn encode_seq_end(&mut self) -> Result<(), Error> {
        let Some(state) = self.state.last() else {
            return Err(Error::Seq);
        };

        let EncoderState::Seq { pos, len } = state else {
            return Err(Error::Seq);
        };

        if pos != len {
            return Err(Error::Seq);
        }

        let _ = self.state.pop();

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_map(&mut self, value: &Map) -> Result<(), Error> {
        self.encode_map_start(value.len())?;

        for (key, value) in value {
            self.encode_any(key)?;
            self.encode_any(value)?;
        }

        self.encode_map_end()
    }

    pub(crate) fn encode_map_value(&mut self, value: &MapValue) -> Result<(), Error> {
        self.encode_map(&value.0)
    }

    pub fn encode_map_start(&mut self, len: usize) -> Result<(), Error> {
        // Push the value's metadata:
        let mut head_byte = MapValue::PREFIX_BIT;
        head_byte |= MapValue::VARIANT_BIT;
        head_byte |= 3; // width exponent of usize (2 ^ 3 = 8)
        self.push_byte(head_byte)?;

        // Push the value's length:
        let neck_bytes = len.to_be_bytes();
        self.push_bytes(&neck_bytes)?;

        self.state.push(EncoderState::map(len));

        Ok(())
    }

    pub fn encode_map_end(&mut self) -> Result<(), Error> {
        let Some(state) = self.state.last() else {
            return Err(Error::Map);
        };

        let EncoderState::Map { pos, len } = state else {
            return Err(Error::Map);
        };

        if pos != len {
            return Err(Error::Map);
        }

        let _ = self.state.pop();

        self.on_encode_value()?;

        Ok(())
    }

    pub fn encode_f32(&mut self, value: f32) -> Result<(), Error> {
        self.encode_float(value)
    }

    pub fn encode_f64(&mut self, value: f64) -> Result<(), Error> {
        self.encode_float(value)
    }

    fn encode_float<T, const N: usize>(&mut self, value: T) -> Result<(), Error>
    where
        T: FloatCore + ToBytes<Bytes = [u8; N]>,
    {
        // Push the value's metadata:
        let mut head_byte = FloatValue::PREFIX_BIT;

        head_byte |= (N as u8) - 1; // width of T, minus 1
        self.push_byte(head_byte)?;

        // Push the value's actual bytes:
        let tail_bytes = value.to_be_bytes();
        self.push_bytes(&tail_bytes)?;

        self.on_encode_value()
    }

    pub(crate) fn encode_float_value(&mut self, value: &FloatValue) -> Result<(), Error> {
        match *value {
            FloatValue::F32(value) => self.encode_f32(value),
            FloatValue::F64(value) => self.encode_f64(value),
        }
    }

    pub fn encode_bytes(&mut self, value: &[u8]) -> Result<(), Error> {
        // Push the value's metadata:
        let mut head_byte = BytesValue::PREFIX_BIT;
        head_byte |= 3; // width exponent of usize (2 ^ 3 = 8)
        self.push_byte(head_byte)?;

        // Push the value's length:
        let neck_bytes = value.len().to_be_bytes();
        self.push_bytes(&neck_bytes)?;

        // Push the value's actual bytes:
        let tail_bytes = value;
        self.push_bytes(tail_bytes)?;

        self.on_encode_value()
    }

    fn encode_bytes_value(&mut self, value: &BytesValue) -> Result<(), Error> {
        self.encode_bytes(&value.0)
    }

    pub fn encode_bool(&mut self, value: bool) -> Result<(), Error> {
        let mut head_byte = BoolValue::PREFIX_BIT;

        if value {
            head_byte |= BoolValue::VALUE_BIT;
        }

        self.push_byte(head_byte)?;

        self.on_encode_value()
    }

    pub fn encode_bool_value(&mut self, value: &BoolValue) -> Result<(), Error> {
        self.encode_bool(value.0)
    }

    pub fn encode_null(&mut self) -> Result<(), Error> {
        let head_byte = NullValue::BIT_REPR;

        self.push_byte(head_byte)?;

        self.on_encode_value()
    }

    fn encode_null_value(&mut self, value: &NullValue) -> Result<(), Error> {
        let NullValue = value;
        self.encode_null()
    }
}

impl Encoder {
    fn push_byte(&mut self, byte: u8) -> Result<(), Error> {
        self.buf.push(byte);
        self.pos += 1;

        Ok(())
    }

    fn push_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.pos += bytes.len();

        self.buf.extend_from_slice(bytes);

        Ok(())
    }

    #[allow(dead_code)]
    fn existing(&self) -> usize {
        self.pos
    }

    fn on_encode_value(&mut self) -> Result<(), Error> {
        if let Some(state) = self.state.last_mut() {
            state.on_encode_value()
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_byte() {
        let mut encoder = Encoder::new(Profile::None);

        encoder.push_byte(1).unwrap();
        assert_eq!(encoder.buf, vec![1]);

        encoder.push_byte(2).unwrap();
        assert_eq!(encoder.buf, vec![1, 2]);

        encoder.push_byte(3).unwrap();
        assert_eq!(encoder.buf, vec![1, 2, 3]);
    }

    #[test]
    fn push_bytes() {
        let mut encoder = Encoder::new(Profile::None);

        encoder.push_bytes(&[1]).unwrap();
        assert_eq!(encoder.buf, vec![1]);

        encoder.push_bytes(&[2, 3]).unwrap();
        assert_eq!(encoder.buf, vec![1, 2, 3]);
    }

    #[test]
    fn existing() {
        let mut encoder = Encoder::new(Profile::None);
        assert_eq!(encoder.existing(), 0);

        encoder.push_byte(42).unwrap();
        assert_eq!(encoder.existing(), 1);
    }

    #[test]
    fn into_vec() {
        let mut encoder = Encoder::new(Profile::None);
        encoder.push_bytes(&[1, 2, 3]).unwrap();

        assert_eq!(encoder.into_vec().unwrap(), vec![1, 2, 3]);
    }
}
