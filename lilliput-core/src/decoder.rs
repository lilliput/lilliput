use num_traits::{FromBytes, PrimInt, Signed, Unsigned};

use crate::{
    num::FromZigZag,
    value::{
        BoolValue, BytesValue, FloatValue, IntValue, Map, MapValue, NullValue, SeqValue,
        StringValue, Value, ValueType,
    },
    Profile,
};

mod bool;
mod bytes;
mod float;
mod map;
mod null;
mod seq;
mod string;

use self::{bool::*, bytes::*, float::*, map::*, null::*, seq::*, string::*};

#[derive(Eq, PartialEq, Debug, thiserror::Error)]
pub enum DecoderError {
    #[error("not a valid UTF-8 string")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("unexpected end of file")]
    Eof,
    #[error("expected type {expected:?}, found {actual:?}")]
    Type {
        expected: ValueType,
        actual: ValueType,
    },
    #[error("incompatible profile")]
    IncompatibleProfile,
    #[error("invalid int")]
    Int,
    #[error("invalid seq")]
    Seq,
    #[error("invalid map")]
    Map,
    #[error("other")]
    Other,
}

#[derive(Eq, PartialEq, Debug)]
enum DecoderState {
    Seq { pos: usize, len: usize },
    Map { pos: usize, len: usize },
}

impl DecoderState {
    fn seq(len: usize) -> Self {
        Self::Seq { pos: 0, len }
    }

    fn map(len: usize) -> Self {
        Self::Map {
            pos: 0,
            len: 2 * len,
        }
    }

    fn on_decode_value(&mut self) -> Result<(), DecoderError> {
        match self {
            DecoderState::Seq { pos, len } => {
                if pos < len {
                    *pos += 1;
                    Ok(())
                } else {
                    Err(DecoderError::Seq)
                }
            }
            DecoderState::Map { pos, len } => {
                if pos < len {
                    *pos += 1;
                    Ok(())
                } else {
                    Err(DecoderError::Map)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Decoder<'a> {
    buf: &'a [u8],
    pos: usize,
    #[allow(dead_code)]
    profile: Profile,
    state: Vec<DecoderState>,
}

impl<'a> Decoder<'a> {
    pub fn new(buf: &'a [u8], profile: Profile) -> Self {
        Decoder {
            buf,
            pos: 0,
            profile,
            state: vec![],
        }
    }
}

impl Decoder<'_> {
    // MARK: - Any Values

    pub fn decode_any(&mut self) -> Result<Value, DecoderError> {
        match ValueType::detect(self.peek_byte()?) {
            ValueType::Int => self.decode_int_value().map(From::from),
            ValueType::String => self.decode_string_value().map(From::from),
            ValueType::Seq => self.decode_seq_value().map(From::from),
            ValueType::Map => self.decode_map_value().map(From::from),
            ValueType::Float => self.decode_float_value().map(From::from),
            ValueType::Bytes => self.decode_bytes_value().map(From::from),
            ValueType::Bool => self.decode_bool_value().map(From::from),
            ValueType::Null => self.decode_null_value().map(From::from),
            ValueType::Reserved => unimplemented!(),
        }
    }

    // MARK: - Int Values

    pub fn decode_u8(&mut self) -> Result<u8, DecoderError> {
        match self.peek_int_size()? {
            1 => self.decode_unsigned::<u8, 1>(),
            2 => Err(DecoderError::Int),
            4 => Err(DecoderError::Int),
            8 => Err(DecoderError::Int),
            _ => unimplemented!(),
        }
    }

    pub fn decode_u16(&mut self) -> Result<u16, DecoderError> {
        match self.peek_int_size()? {
            1 => Ok(self.decode_unsigned::<u8, 1>()?.into()),
            2 => self.decode_unsigned::<u16, 2>(),
            4 => Err(DecoderError::Int),
            8 => Err(DecoderError::Int),
            _ => unimplemented!(),
        }
    }

    pub fn decode_u32(&mut self) -> Result<u32, DecoderError> {
        match self.peek_int_size()? {
            1 => Ok(self.decode_unsigned::<u8, 1>()?.into()),
            2 => Ok(self.decode_unsigned::<u16, 2>()?.into()),
            4 => self.decode_unsigned::<u32, 4>(),
            8 => Err(DecoderError::Int),
            _ => unimplemented!(),
        }
    }

    pub fn decode_u64(&mut self) -> Result<u64, DecoderError> {
        match self.peek_int_size()? {
            1 => Ok(self.decode_unsigned::<u8, 1>()?.into()),
            2 => Ok(self.decode_unsigned::<u16, 2>()?.into()),
            4 => Ok(self.decode_unsigned::<u32, 4>()?.into()),
            8 => self.decode_unsigned::<u64, 8>(),
            _ => unimplemented!(),
        }
    }

    pub fn decode_i8(&mut self) -> Result<i8, DecoderError> {
        match self.peek_int_size()? {
            1 => self.decode_signed::<i8, u8, 1>(),
            2 => Err(DecoderError::Int),
            4 => Err(DecoderError::Int),
            8 => Err(DecoderError::Int),
            _ => unimplemented!(),
        }
    }

    pub fn decode_i16(&mut self) -> Result<i16, DecoderError> {
        match self.peek_int_size()? {
            1 => Ok(self.decode_signed::<i8, u8, 1>()?.into()),
            2 => self.decode_signed::<i16, u16, 2>(),
            4 => Err(DecoderError::Int),
            8 => Err(DecoderError::Int),
            _ => unimplemented!(),
        }
    }

    pub fn decode_i32(&mut self) -> Result<i32, DecoderError> {
        match self.peek_int_size()? {
            1 => Ok(self.decode_signed::<i8, u8, 1>()?.into()),
            2 => Ok(self.decode_signed::<i16, u16, 2>()?.into()),
            4 => self.decode_signed::<i32, u32, 4>(),
            8 => Err(DecoderError::Int),
            _ => unimplemented!(),
        }
    }

    pub fn decode_i64(&mut self) -> Result<i64, DecoderError> {
        match self.peek_int_size()? {
            1 => Ok(self.decode_signed::<i8, u8, 1>()?.into()),
            2 => Ok(self.decode_signed::<i16, u16, 2>()?.into()),
            4 => Ok(self.decode_signed::<i32, u32, 4>()?.into()),
            8 => self.decode_signed::<i64, u64, 8>(),
            _ => unimplemented!(),
        }
    }

    fn decode_signed<S, U, const N: usize>(&mut self) -> Result<S, DecoderError>
    where
        S: Signed + PrimInt + FromZigZag<ZigZag = U>,
        U: FromBytes<Bytes = [u8; N]>,
    {
        let byte = self.pull_byte_expecting_type(ValueType::Int)?;
        let is_long = byte & IntValue::VARIANT_BIT != 0b0;
        let is_signed = byte & IntValue::SIGNEDNESS_BIT != 0b0;

        if !is_signed {
            return Err(DecoderError::Other);
        }

        if is_long {
            let is_valid = byte & IntValue::LONG_RESERVED_BITS == 0b0;
            assert!(is_valid, "padding bits should be zero");

            let size_len = (byte & IntValue::LONG_WIDTH_BITS) as usize + 1;

            if size_len > N {
                return Err(DecoderError::Int);
            }

            let pulled_bytes = self.pull_bytes(size_len)?;

            let mut bytes: [u8; N] = [0b0; N];
            bytes.copy_from_slice(pulled_bytes);

            let unsigned = U::from_be_bytes(&bytes);
            let signed = S::from_zig_zag(unsigned);

            self.on_decode_value()?;

            Ok(signed)
        } else {
            Err(DecoderError::IncompatibleProfile)
        }
    }

    fn decode_unsigned<T, const N: usize>(&mut self) -> Result<T, DecoderError>
    where
        T: Unsigned + PrimInt + FromBytes<Bytes = [u8; N]>,
    {
        let byte = self.pull_byte_expecting_type(ValueType::Int)?;
        let is_long = byte & IntValue::VARIANT_BIT != 0b0;
        let is_signed = byte & IntValue::SIGNEDNESS_BIT != 0b0;

        if is_signed {
            return Err(DecoderError::Other);
        }

        if is_long {
            let is_valid = byte & IntValue::LONG_RESERVED_BITS == 0b0;
            assert!(is_valid, "padding bits should be zero");

            let size_len = (byte & IntValue::LONG_WIDTH_BITS) as usize + 1;

            if size_len > N {
                return Err(DecoderError::Int);
            }

            let pulled_bytes = self.pull_bytes(size_len)?;

            let mut bytes: [u8; N] = [0b0; N];
            bytes.copy_from_slice(pulled_bytes);

            let unsigned = T::from_be_bytes(&bytes);

            self.on_decode_value()?;

            Ok(unsigned)
        } else {
            Err(DecoderError::IncompatibleProfile)
        }
    }

    pub fn decode_int_value(&mut self) -> Result<IntValue, DecoderError> {
        let byte = self.peek_byte_expecting_type(ValueType::Int)?;
        let is_long = byte & IntValue::VARIANT_BIT != 0b0;
        let is_signed = byte & IntValue::SIGNEDNESS_BIT != 0b0;

        if is_long {
            let size_len = (byte & IntValue::LONG_WIDTH_BITS) as usize + 1;

            if is_signed {
                match size_len {
                    1 => Ok(IntValue::Signed(self.decode_signed::<i8, u8, 1>()?.into())),
                    2 => Ok(IntValue::Signed(
                        self.decode_signed::<i16, u16, 2>()?.into(),
                    )),
                    4 => Ok(IntValue::Signed(
                        self.decode_signed::<i32, u32, 4>()?.into(),
                    )),
                    8 => Ok(IntValue::Signed(
                        self.decode_signed::<i64, u64, 8>()?.into(),
                    )),
                    _ => Err(DecoderError::IncompatibleProfile),
                }
            } else {
                match size_len {
                    1 => Ok(IntValue::Unsigned(self.decode_unsigned::<u8, 1>()?.into())),
                    2 => Ok(IntValue::Unsigned(self.decode_unsigned::<u16, 2>()?.into())),
                    4 => Ok(IntValue::Unsigned(self.decode_unsigned::<u32, 4>()?.into())),
                    8 => Ok(IntValue::Unsigned(self.decode_unsigned::<u64, 8>()?.into())),
                    _ => Err(DecoderError::IncompatibleProfile),
                }
            }
        } else {
            Err(DecoderError::IncompatibleProfile)
        }
    }

    fn peek_int_size(&self) -> Result<usize, DecoderError> {
        let byte = self.peek_byte_expecting_type(ValueType::Int)?;
        let is_long = byte & IntValue::VARIANT_BIT != 0b0;

        if is_long {
            Ok((byte & IntValue::LONG_WIDTH_BITS) as usize + 1)
        } else {
            Ok(1)
        }
    }

    // MARK: - String Values

    pub fn decode_string(&mut self) -> Result<String, DecoderError> {
        let value = StringDecoder::with(self).decode_string()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub(crate) fn decode_string_value(&mut self) -> Result<StringValue, DecoderError> {
        let value = StringDecoder::with(self).decode_string_value()?;

        self.on_decode_value()?;

        Ok(value)
    }

    // MARK: - Seq Values

    pub fn decode_seq(&mut self) -> Result<Vec<Value>, DecoderError> {
        let value = SeqDecoder::with(self).decode_seq()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub(crate) fn decode_seq_value(&mut self) -> Result<SeqValue, DecoderError> {
        let value = SeqDecoder::with(self).decode_seq_value()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub fn decode_seq_start(&mut self) -> Result<usize, DecoderError> {
        let len = SeqDecoder::with(self).decode_seq_start()?;

        self.state.push(DecoderState::seq(len));

        Ok(len)
    }

    pub fn decode_seq_end(&mut self) -> Result<(), DecoderError> {
        let Some(state) = self.state.pop() else {
            return Err(DecoderError::Seq);
        };

        let DecoderState::Seq { pos, len } = state else {
            return Err(DecoderError::Seq);
        };

        if pos != len {
            return Err(DecoderError::Seq);
        }

        SeqDecoder::with(self).decode_seq_end()?;

        self.on_decode_value()?;

        Ok(())
    }

    // MARK: - Map Values

    pub fn decode_map(&mut self) -> Result<Map, DecoderError> {
        let value = MapDecoder::with(self).decode_map()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub(crate) fn decode_map_value(&mut self) -> Result<MapValue, DecoderError> {
        let value = MapDecoder::with(self).decode_map_value()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub fn decode_map_start(&mut self) -> Result<usize, DecoderError> {
        let len = MapDecoder::with(self).decode_map_start()?;

        self.state.push(DecoderState::map(len));

        Ok(len)
    }

    pub fn decode_map_end(&mut self) -> Result<(), DecoderError> {
        let Some(state) = self.state.pop() else {
            return Err(DecoderError::Map);
        };

        let DecoderState::Map { pos, len } = state else {
            return Err(DecoderError::Map);
        };

        if pos != len {
            return Err(DecoderError::Map);
        }

        MapDecoder::with(self).decode_map_end()?;

        self.on_decode_value()?;

        Ok(())
    }

    // MARK: - Float Values

    pub fn decode_f32(&mut self) -> Result<f32, DecoderError> {
        let value = FloatDecoder::with(self).decode_float()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub fn decode_f64(&mut self) -> Result<f64, DecoderError> {
        let value = FloatDecoder::with(self).decode_float()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub fn decode_float_value(&mut self) -> Result<FloatValue, DecoderError> {
        let value = FloatDecoder::with(self).decode_float_value()?;

        self.on_decode_value()?;

        Ok(value)
    }

    // MARK: - Bytes Values

    pub fn decode_bytes(&mut self) -> Result<Vec<u8>, DecoderError> {
        let value = BytesDecoder::with(self).decode_bytes()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub fn decode_bytes_value(&mut self) -> Result<BytesValue, DecoderError> {
        let value = BytesDecoder::with(self).decode_bytes_value()?;

        self.on_decode_value()?;

        Ok(value)
    }

    // MARK: - Bool Values

    pub fn decode_bool(&mut self) -> Result<bool, DecoderError> {
        let value = BoolDecoder::with(self).decode_bool()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub fn decode_bool_value(&mut self) -> Result<BoolValue, DecoderError> {
        let value = BoolDecoder::with(self).decode_bool_value()?;

        self.on_decode_value()?;

        Ok(value)
    }

    // MARK: - Null Values

    pub fn decode_null(&mut self) -> Result<(), DecoderError> {
        NullDecoder::with(self).decode_null()?;

        self.on_decode_value()?;

        Ok(())
    }

    pub fn decode_null_value(&mut self) -> Result<NullValue, DecoderError> {
        let value = NullDecoder::with(self).decode_null_value()?;

        self.on_decode_value()?;

        Ok(value)
    }
}

// MARK: - Auxiliary Methods

impl Decoder<'_> {
    fn peek_byte(&self) -> Result<u8, DecoderError> {
        if self.eof() {
            return Err(DecoderError::Eof);
        }

        Ok(self.buf[self.pos])
    }

    fn peek_byte_expecting_type(&self, expected: ValueType) -> Result<u8, DecoderError> {
        let byte = self.peek_byte()?;

        let actual = ValueType::detect(byte);

        if actual == expected {
            Ok(byte)
        } else {
            Err(DecoderError::Type { expected, actual })
        }
    }

    fn pull_byte_expecting_type(&mut self, expected: ValueType) -> Result<u8, DecoderError> {
        let byte = self.peek_byte_expecting_type(expected)?;

        self.pos += 1;

        Ok(byte)
    }

    fn pull_bytes(&mut self, len: usize) -> Result<&[u8], DecoderError> {
        if self.pos + len > self.buf.len() {
            return Err(DecoderError::Eof);
        }

        let range = self.pos..(self.pos + len);

        self.pos += len;

        Ok(&self.buf[range])
    }

    fn remaining_len(&self) -> usize {
        self.buf.len() - self.pos
    }

    #[allow(dead_code)]
    fn remaining(&self) -> &[u8] {
        &self.buf[self.pos..]
    }

    fn eof(&self) -> bool {
        self.pos >= self.buf.len()
    }

    fn on_decode_value(&mut self) -> Result<(), DecoderError> {
        if let Some(state) = self.state.last_mut() {
            state.on_decode_value()
        } else {
            Ok(())
        }
    }
}

// MARK: - Tests

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let bytes = vec![1, 2, 3];
        let decoder = Decoder::new(&bytes, Profile::None);
        assert_eq!(decoder.buf, vec![1, 2, 3]);
        assert_eq!(decoder.pos, 0);
        assert_eq!(decoder.profile, Profile::None);
        assert_eq!(decoder.state.len(), 0);
    }

    #[test]
    fn pull_bytes() {
        let bytes = vec![1, 2, 3];
        let mut decoder = Decoder::new(&bytes, Profile::None);
        assert_eq!(decoder.remaining(), &[1, 2, 3]);
        assert_eq!(decoder.pos, 0);
        assert_eq!(decoder.remaining_len(), 3);

        assert_eq!(decoder.pull_bytes(0).unwrap(), &[]);
        assert_eq!(decoder.remaining(), &[1, 2, 3]);
        assert_eq!(decoder.pos, 0);
        assert_eq!(decoder.remaining_len(), 3);

        assert_eq!(decoder.pull_bytes(1).unwrap(), &[1]);
        assert_eq!(decoder.remaining(), &[2, 3]);
        assert_eq!(decoder.pos, 1);
        assert_eq!(decoder.remaining_len(), 2);

        assert_eq!(decoder.pull_bytes(2).unwrap(), &[2, 3]);
        assert_eq!(decoder.remaining(), &[]);
        assert_eq!(decoder.pos, 3);
        assert_eq!(decoder.remaining_len(), 0);

        assert_eq!(decoder.pull_bytes(3).unwrap_err(), DecoderError::Eof);
        assert_eq!(decoder.remaining(), &[]);
        assert_eq!(decoder.pos, 3);
        assert_eq!(decoder.remaining_len(), 0);
    }

    #[test]
    fn remaining_len() {
        let bytes = vec![1, 2, 3];
        let decoder = Decoder::new(&bytes, Profile::None);
        assert_eq!(decoder.remaining_len(), 3);
    }

    #[test]
    fn remaining() {
        let bytes = vec![1, 2, 3];
        let decoder = Decoder::new(&bytes, Profile::None);
        assert_eq!(decoder.remaining(), &[1, 2, 3]);
    }
}
