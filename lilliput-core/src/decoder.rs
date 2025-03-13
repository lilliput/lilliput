use crate::{
    Profile,
    num::{FromFloat, IntoFloat},
    value::{
        BoolValue, BytesValue, FloatValue, Map, MapValue, NullValue, SeqValue, StringValue, Value,
        ValueType,
    },
};

#[derive(Eq, PartialEq, Debug, thiserror::Error)]
pub enum Error {
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
    #[error("invalid seq")]
    Seq,
    #[error("invalid map")]
    Map,
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

    fn on_decode_value(&mut self) -> Result<(), Error> {
        match self {
            DecoderState::Seq { pos, len } => {
                if pos < len {
                    *pos += 1;
                    Ok(())
                } else {
                    Err(Error::Seq)
                }
            }
            DecoderState::Map { pos, len } => {
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
pub struct Decoder<'a> {
    buf: &'a [u8],
    pos: usize,
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

impl<'a> Decoder<'a> {
    pub fn decode_any(&mut self) -> Result<Value, Error> {
        match ValueType::detect(self.peek_byte()?) {
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

    pub fn decode_string(&mut self) -> Result<String, Error> {
        let byte = self.pull_byte_expecting_type(ValueType::String)?;

        let is_long = byte & StringValue::VARIANT_BIT != 0b0;

        if is_long {
            let is_valid = byte & StringValue::LONG_RESERVED_BITS == 0b0;
            let len_width = (byte & StringValue::LONG_LEN_WIDTH_BITS) as usize + 1;

            assert!(is_valid, "padding bits should be zero");

            let mut bytes: [u8; 8] = [0b0; 8];

            let range = {
                let start = 8 - len_width;
                let end = start + len_width;
                start..end
            };

            bytes[range].copy_from_slice(self.pull_bytes(len_width)?);

            let len = u64::from_be_bytes(bytes) as usize;

            let mut bytes = Vec::with_capacity(len.min(self.remaining_len()));
            bytes.extend_from_slice(self.pull_bytes(len)?);

            let value = String::from_utf8(bytes)?;

            self.on_decode_value()?;

            Ok(value)
        } else {
            Err(Error::IncompatibleProfile)
        }
    }

    pub(crate) fn decode_string_value(&mut self) -> Result<StringValue, Error> {
        self.decode_string().map(From::from)
    }

    pub fn decode_seq(&mut self) -> Result<Vec<Value>, Error> {
        let len = self.decode_seq_start()?;
        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            let value = self.decode_any()?;
            vec.push(value);
        }

        self.decode_seq_end()?;

        Ok(vec)
    }

    pub(crate) fn decode_seq_value(&mut self) -> Result<SeqValue, Error> {
        self.decode_seq().map(From::from)
    }

    pub fn decode_seq_start(&mut self) -> Result<usize, Error> {
        let byte = self.pull_byte_expecting_type(ValueType::Seq)?;

        let is_long = byte & SeqValue::VARIANT_BIT != 0b0;

        if is_long {
            let is_valid = byte & SeqValue::LONG_RESERVED_BIT == 0b0;
            let len_width = (byte & SeqValue::LONG_LEN_WIDTH_BITS) as usize + 1;

            assert!(is_valid, "padding bits should be zero");

            let mut bytes: [u8; 8] = [0b0; 8];

            let range = {
                let start = 8 - len_width;
                let end = start + len_width;
                start..end
            };

            bytes[range].copy_from_slice(self.pull_bytes(len_width)?);

            let len = u64::from_be_bytes(bytes) as usize;

            if self.remaining_len() < len {
                return Err(Error::Eof);
            }

            self.state.push(DecoderState::seq(len));

            Ok(len)
        } else {
            Err(Error::IncompatibleProfile)
        }
    }

    pub fn decode_seq_end(&mut self) -> Result<(), Error> {
        let Some(state) = self.state.last() else {
            return Err(Error::Seq);
        };

        let DecoderState::Seq { pos, len } = state else {
            return Err(Error::Seq);
        };

        if pos != len {
            return Err(Error::Seq);
        }

        let _ = self.state.pop();

        self.on_decode_value()?;

        Ok(())
    }

    pub fn decode_map(&mut self) -> Result<Map, Error> {
        let len = self.decode_map_start()?;

        #[cfg(feature = "preserve_order")]
        pub(crate) type Map = ordermap::OrderMap<Value, Value>;

        #[cfg(not(feature = "preserve_order"))]
        pub(crate) type Map = std::collections::BTreeMap<Value, Value>;

        let mut map = Map::default();

        for _ in 0..len {
            let key = self.decode_any()?;
            let value = self.decode_any()?;
            map.insert(key, value);
        }

        let () = self.decode_map_end()?;

        Ok(map)
    }

    pub(crate) fn decode_map_value(&mut self) -> Result<MapValue, Error> {
        self.decode_map().map(From::from)
    }

    pub fn decode_map_start(&mut self) -> Result<usize, Error> {
        let byte = self.pull_byte_expecting_type(ValueType::Map)?;

        let is_long = byte & MapValue::VARIANT_BIT != 0b0;

        if is_long {
            let len_width_exponent = (byte & MapValue::LONG_LEN_WIDTH_BITS) as u32;
            let len_width = 1_usize << len_width_exponent;

            let mut bytes: [u8; 8] = [0b0; 8];

            let range = {
                let start = 8 - len_width;
                let end = start + len_width;
                start..end
            };

            bytes[range].copy_from_slice(self.pull_bytes(len_width)?);

            let len = u64::from_be_bytes(bytes) as usize;

            if self.remaining_len() < len {
                return Err(Error::Eof);
            }

            self.state.push(DecoderState::map(len));

            Ok(len)
        } else {
            Err(Error::IncompatibleProfile)
        }
    }

    pub fn decode_map_end(&mut self) -> Result<(), Error> {
        let Some(state) = self.state.last() else {
            return Err(Error::Map);
        };

        let DecoderState::Map { pos, len } = state else {
            return Err(Error::Map);
        };

        if pos != len {
            return Err(Error::Map);
        }

        let _ = self.state.pop();

        self.on_decode_value()?;

        Ok(())
    }

    pub fn decode_f32(&mut self) -> Result<f32, Error> {
        self.decode_float()
    }

    pub fn decode_f64(&mut self) -> Result<f64, Error> {
        self.decode_float()
    }

    fn decode_float<T>(&mut self) -> Result<T, Error>
    where
        T: FromFloat<f32> + FromFloat<f64>,
    {
        let byte = self.pull_byte_expecting_type(ValueType::Float)?;

        let width = (byte & FloatValue::WIDTH_BITS) as usize + 1;

        match width {
            4 => {
                let mut bytes: [u8; 4] = [0b0; 4];
                bytes.copy_from_slice(self.pull_bytes(width)?);

                self.on_decode_value()?;

                Ok(f32::from_be_bytes(bytes).into_float())
            }
            8 => {
                let mut bytes: [u8; 8] = [0b0; 8];
                bytes.copy_from_slice(self.pull_bytes(width)?);

                self.on_decode_value()?;

                Ok(f64::from_be_bytes(bytes).into_float())
            }
            _ => Err(Error::IncompatibleProfile),
        }
    }

    pub(crate) fn decode_float_value(&mut self) -> Result<FloatValue, Error> {
        let byte = self.peek_byte_expecting_type(ValueType::Float)?;

        let width = (byte & FloatValue::WIDTH_BITS) as usize + 1;

        match width {
            4 => self.decode_f32().map(FloatValue::F32),
            8 => self.decode_f64().map(FloatValue::F64),
            _ => Err(Error::IncompatibleProfile),
        }
    }

    pub fn decode_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let byte = self.pull_byte_expecting_type(ValueType::Bytes)?;

        let len_width_exponent = (byte & BytesValue::LONG_WIDTH_BITS) as u32;
        let len_width = 1_usize << len_width_exponent;

        let len = {
            let range = {
                let start = 8 - len_width;
                let end = start + len_width;
                start..end
            };

            let mut len_bytes: [u8; 8] = [0b0; 8];
            len_bytes[range].copy_from_slice(self.pull_bytes(len_width)?);

            u64::from_be_bytes(len_bytes) as usize
        };

        let mut bytes = Vec::with_capacity(len.min(self.remaining_len()));
        bytes.extend_from_slice(self.pull_bytes(len)?);

        self.on_decode_value()?;

        Ok(bytes)
    }

    fn decode_bytes_value(&mut self) -> Result<BytesValue, Error> {
        self.decode_bytes().map(From::from)
    }

    pub fn decode_bool(&mut self) -> Result<bool, Error> {
        let byte = self.pull_byte_expecting_type(ValueType::Bool)?;

        let value = byte & BoolValue::VALUE_BIT != 0b0;

        self.on_decode_value()?;

        Ok(value)
    }

    fn decode_bool_value(&mut self) -> Result<BoolValue, Error> {
        self.decode_bool().map(From::from)
    }

    pub fn decode_null(&mut self) -> Result<(), Error> {
        let _byte = self.pull_byte_expecting_type(ValueType::Null)?;

        self.on_decode_value()?;

        Ok(())
    }

    fn decode_null_value(&mut self) -> Result<NullValue, Error> {
        self.decode_null()?;

        Ok(NullValue)
    }
}

impl<'a> Decoder<'a> {
    fn peek_byte(&self) -> Result<u8, Error> {
        if self.eof() {
            return Err(Error::Eof);
        }

        Ok(self.buf[self.pos])
    }

    fn peek_byte_expecting_type(&self, expected: ValueType) -> Result<u8, Error> {
        let byte = self.peek_byte()?;

        let actual = ValueType::detect(byte);

        if actual == expected {
            Ok(byte)
        } else {
            Err(Error::Type { expected, actual })
        }
    }

    fn pull_byte_expecting_type(&mut self, expected: ValueType) -> Result<u8, Error> {
        let byte = self.peek_byte_expecting_type(expected)?;

        self.pos += 1;

        Ok(byte)
    }

    fn pull_bytes(&mut self, len: usize) -> Result<&[u8], Error> {
        if self.pos + len > self.buf.len() {
            return Err(Error::Eof);
        }

        let range = self.pos..(self.pos + len);

        self.pos += len;

        Ok(&self.buf[range])
    }

    fn remaining_len(&self) -> usize {
        self.buf.len() - self.pos
    }

    fn remaining(&self) -> &[u8] {
        &self.buf[self.pos..]
    }

    fn eof(&self) -> bool {
        self.pos >= self.buf.len()
    }

    fn on_decode_value(&mut self) -> Result<(), Error> {
        if let Some(state) = self.state.last_mut() {
            state.on_decode_value()
        } else {
            Ok(())
        }
    }
}

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

        assert_eq!(decoder.pull_bytes(3).unwrap_err(), Error::Eof);
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
