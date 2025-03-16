use crate::{
    header::{DecodeHeader, HeaderDecodeError, HeaderType},
    num::TryFromInt,
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
pub enum DecoderError {
    #[error("not a valid UTF-8 string")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("unexpected end of file")]
    Eof,
    #[error(transparent)]
    Header(#[from] HeaderDecodeError),
    #[error("insufficient profile {profile:?}")]
    Profile { profile: Profile },
    #[error("invalid int")]
    Int(#[from] IntDecoderError),
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
        match HeaderType::detect(self.peek_byte()?) {
            HeaderType::Int => self.decode_int_value().map(From::from),
            HeaderType::String => self.decode_string_value().map(From::from),
            HeaderType::Seq => self.decode_seq_value().map(From::from),
            HeaderType::Map => self.decode_map_value().map(From::from),
            HeaderType::Float => self.decode_float_value().map(From::from),
            HeaderType::Bytes => self.decode_bytes_value().map(From::from),
            HeaderType::Bool => self.decode_bool_value().map(From::from),
            HeaderType::Null => self.decode_null_value().map(From::from),
            HeaderType::Reserved => {
                unimplemented!()
            }
        }
    }

    // MARK: - Int Values

    pub fn decode_u8(&mut self) -> Result<u8, DecoderError> {
        let value = IntDecoder::with(self).decode_unsigned()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub fn decode_u16(&mut self) -> Result<u16, DecoderError> {
        let value = IntDecoder::with(self).decode_unsigned()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub fn decode_u32(&mut self) -> Result<u32, DecoderError> {
        let value = IntDecoder::with(self).decode_unsigned()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub fn decode_u64(&mut self) -> Result<u64, DecoderError> {
        let value = IntDecoder::with(self).decode_unsigned()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub fn decode_i8(&mut self) -> Result<i8, DecoderError> {
        let value = IntDecoder::with(self).decode_signed()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub fn decode_i16(&mut self) -> Result<i16, DecoderError> {
        let value = IntDecoder::with(self).decode_signed()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub fn decode_i32(&mut self) -> Result<i32, DecoderError> {
        let value = IntDecoder::with(self).decode_signed()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub fn decode_i64(&mut self) -> Result<i64, DecoderError> {
        let value = IntDecoder::with(self).decode_signed()?;

        self.on_decode_value()?;

        Ok(value)
    }

    pub fn decode_int_value(&mut self) -> Result<IntValue, DecoderError> {
        let value = IntDecoder::with(self).decode_int_value()?;

        self.on_decode_value()?;

        Ok(value)
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

    fn peek_header<T>(&mut self) -> Result<T, DecoderError>
    where
        T: DecodeHeader,
    {
        let byte = self.peek_byte()?;

        let header = T::decode(byte)?;

        Ok(header)
    }

    fn pull_header<T>(&mut self) -> Result<T, DecoderError>
    where
        T: DecodeHeader,
    {
        let header = self.peek_header()?;

        self.pos += 1;

        Ok(header)
    }

    fn pull_bytes(&mut self, len: usize) -> Result<&[u8], DecoderError> {
        if self.pos + len > self.buf.len() {
            return Err(DecoderError::Eof);
        }

        let range = self.pos..(self.pos + len);

        self.pos += len;

        Ok(&self.buf[range])
    }

    fn pull_fixed_bytes<const N: usize>(&mut self) -> Result<&[u8; N], DecoderError> {
        let bytes = self.pull_bytes(N)?;
        Ok(bytes.try_into().unwrap())
    }

    #[inline(always)]
    fn pull_len_bytes(&mut self, len_width: usize) -> Result<usize, DecoderError> {
        match len_width {
            1 => {
                let len_bytes: &[u8; 1] = self.pull_fixed_bytes()?;
                let len = u8::from_be_bytes(*len_bytes);
                Ok(usize::try_from_int(len).map_err(IntDecoderError::OutOfBounds)?)
            }
            2 => {
                let len_bytes: &[u8; 2] = self.pull_fixed_bytes()?;
                let len = u16::from_be_bytes(*len_bytes);
                Ok(usize::try_from_int(len).map_err(IntDecoderError::OutOfBounds)?)
            }
            3 => {
                const MAX_LEN_WIDTH: usize = 4;
                let mut len_bytes: [u8; MAX_LEN_WIDTH] = [0b0; MAX_LEN_WIDTH];
                let len_width_start = MAX_LEN_WIDTH - len_width;
                len_bytes[len_width_start..].copy_from_slice(self.pull_bytes(len_width)?);
                let len = u32::from_be_bytes(len_bytes);
                Ok(usize::try_from_int(len).map_err(IntDecoderError::OutOfBounds)?)
            }
            4 => {
                let len_bytes: &[u8; 4] = self.pull_fixed_bytes()?;
                let len = u32::from_be_bytes(*len_bytes);
                Ok(usize::try_from_int(len).map_err(IntDecoderError::OutOfBounds)?)
            }
            5..=7 => {
                const MAX_LEN_WIDTH: usize = 8;
                let mut len_bytes: [u8; MAX_LEN_WIDTH] = [0b0; MAX_LEN_WIDTH];
                let len_width_start = MAX_LEN_WIDTH - len_width;
                len_bytes[len_width_start..].copy_from_slice(self.pull_bytes(len_width)?);
                let len = u64::from_be_bytes(len_bytes);
                Ok(usize::try_from_int(len).map_err(IntDecoderError::OutOfBounds)?)
            }
            8 => {
                let len_bytes: &[u8; 8] = self.pull_fixed_bytes()?;
                let len = u64::from_be_bytes(*len_bytes);
                Ok(usize::try_from_int(len).map_err(IntDecoderError::OutOfBounds)?)
            }
            _ => unreachable!(),
        }
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
