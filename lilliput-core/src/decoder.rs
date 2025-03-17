use crate::{
    header::{DecodeHeader, HeaderDecodeError, HeaderType},
    io::BufRead,
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
    #[error("not a valid UTF-8 sequence")]
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
    #[error("reader error: {0}")]
    Reader(String),
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
pub struct Decoder<R> {
    reader: R,
    pos: usize,
    #[allow(dead_code)]
    profile: Profile,
    state: Vec<DecoderState>,
}

impl<R> Decoder<R> {
    pub fn new(reader: R, profile: Profile) -> Self {
        Decoder {
            reader,
            pos: 0,
            profile,
            state: vec![],
        }
    }
}

impl<R> Decoder<R>
where
    R: BufRead,
{
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

impl<R> Decoder<R>
where
    R: BufRead,
{
    fn peek_byte(&mut self) -> Result<u8, DecoderError> {
        let peek_buf = self
            .reader
            .fill_buf()
            .map_err(|err| DecoderError::Reader(err.to_string()))?;

        if let Some(&byte) = peek_buf.first() {
            Ok(byte)
        } else {
            Err(DecoderError::Eof)
        }
    }

    fn peek_bytes(&mut self) -> Result<&[u8], DecoderError> {
        self.reader
            .fill_buf()
            .map_err(|err| DecoderError::Reader(err.to_string()))
    }

    fn consume_bytes(&mut self, len: usize) -> Result<(), DecoderError> {
        self.reader.consume(len);
        self.pos += 1;

        Ok(())
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

        self.consume_bytes(1)?;

        Ok(header)
    }

    fn pull_bytes(&mut self, buf: &mut [u8]) -> Result<usize, DecoderError> {
        self.reader
            .read(buf)
            .map_err(|err| DecoderError::Reader(err.to_string()))
    }

    fn pull_bytes_exact(&mut self, buf: &mut [u8]) -> Result<(), DecoderError> {
        let mut buf_pos = 0;

        while buf_pos < buf.len() {
            let read_len = self.pull_bytes(&mut buf[buf_pos..])?;

            if read_len == 0 {
                return Err(DecoderError::Eof);
            }

            self.pos += read_len;
            buf_pos += read_len;
        }

        Ok(())
    }

    #[inline(always)]
    fn pull_len_bytes(&mut self, len_width: usize) -> Result<usize, DecoderError> {
        match len_width {
            1 => {
                let mut len_bytes: [u8; 1] = [0; 1];
                self.pull_bytes_exact(&mut len_bytes)?;
                let len = u8::from_be_bytes(len_bytes);
                Ok(usize::try_from_int(len).map_err(IntDecoderError::OutOfBounds)?)
            }
            2 => {
                let mut len_bytes: [u8; 2] = [0; 2];
                self.pull_bytes_exact(&mut len_bytes)?;
                let len = u16::from_be_bytes(len_bytes);
                Ok(usize::try_from_int(len).map_err(IntDecoderError::OutOfBounds)?)
            }
            3 => {
                const MAX_LEN_WIDTH: usize = 4;
                let mut len_bytes: [u8; MAX_LEN_WIDTH] = [0b0; MAX_LEN_WIDTH];
                let len_width_start = MAX_LEN_WIDTH - len_width;
                self.pull_bytes_exact(&mut len_bytes[len_width_start..])?;
                let len = u32::from_be_bytes(len_bytes);
                Ok(usize::try_from_int(len).map_err(IntDecoderError::OutOfBounds)?)
            }
            4 => {
                let mut len_bytes: [u8; 4] = [0; 4];
                self.pull_bytes_exact(&mut len_bytes)?;
                let len = u32::from_be_bytes(len_bytes);
                Ok(usize::try_from_int(len).map_err(IntDecoderError::OutOfBounds)?)
            }
            5..=7 => {
                const MAX_LEN_WIDTH: usize = 8;
                let mut len_bytes: [u8; MAX_LEN_WIDTH] = [0b0; MAX_LEN_WIDTH];
                let len_width_start = MAX_LEN_WIDTH - len_width;
                self.pull_bytes_exact(&mut len_bytes[len_width_start..])?;
                let len = u64::from_be_bytes(len_bytes);
                Ok(usize::try_from_int(len).map_err(IntDecoderError::OutOfBounds)?)
            }
            8 => {
                let mut len_bytes: [u8; 8] = [0; 8];
                self.pull_bytes_exact(&mut len_bytes)?;
                let len = u64::from_be_bytes(len_bytes);
                Ok(usize::try_from_int(len).map_err(IntDecoderError::OutOfBounds)?)
            }
            _ => unreachable!(),
        }
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
    use crate::io::StdIoBufReader;

    use super::*;

    #[test]
    fn new() {
        let bytes: StdIoBufReader<&[u8]> = StdIoBufReader(&[1, 2, 3]);
        let decoder = Decoder::new(&bytes, Profile::None);
        assert_eq!(decoder.reader.0, vec![1, 2, 3]);
        assert_eq!(decoder.pos, 0);
        assert_eq!(decoder.profile, Profile::None);
        assert_eq!(decoder.state.len(), 0);
    }

    #[test]
    fn pull_bytes_exact() {
        let bytes: StdIoBufReader<&[u8]> = StdIoBufReader(&[1, 2, 3]);
        let mut decoder = Decoder::new(bytes, Profile::None);
        assert_eq!(decoder.pos, 0);

        let mut buf = vec![];
        decoder.pull_bytes_exact(&mut buf).unwrap();
        assert_eq!(buf, &[]);
        assert_eq!(decoder.pos, 0);

        let mut buf = vec![0];
        decoder.pull_bytes_exact(&mut buf).unwrap();
        assert_eq!(buf, &[1]);
        assert_eq!(decoder.pos, 1);

        let mut buf = vec![0, 0];
        decoder.pull_bytes_exact(&mut buf).unwrap();
        assert_eq!(buf, &[2, 3]);
        assert_eq!(decoder.pos, 3);

        let mut buf = vec![0, 0, 0];
        assert_eq!(
            decoder.pull_bytes_exact(&mut buf).unwrap_err(),
            DecoderError::Eof
        );
        assert_eq!(decoder.pos, 3);
    }
}
