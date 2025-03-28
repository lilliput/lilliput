use crate::{
    error::{Error, Result},
    header::{DecodeHeader, Header},
    io::{Read, Reference},
    marker::Marker,
    value::Value,
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
pub struct Decoder<R> {
    reader: R,
    pos: usize,
    peeked: Option<u8>,
}

impl<R> Decoder<R> {
    pub fn new(reader: R) -> Self {
        Decoder {
            reader,
            pos: 0,
            peeked: None,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    // MARK: - Any Values

    pub fn peek_marker(&mut self) -> Result<Marker> {
        Ok(self.peek_header::<Header>()?.marker())
    }

    pub fn decode_any(&mut self) -> Result<Value> {
        match self.peek_header()? {
            Header::Int(_) => self.decode_int_value().map(From::from),
            Header::String(_) => self.decode_string_value().map(From::from),
            Header::Seq(_) => self.decode_seq_value().map(From::from),
            Header::Map(_) => self.decode_map_value().map(From::from),
            Header::Float(_) => self.decode_float_value().map(From::from),
            Header::Bytes(_) => self.decode_bytes_value().map(From::from),
            Header::Bool(_) => self.decode_bool_value().map(From::from),
            Header::Null(_) => self.decode_null_value().map(From::from),
        }
    }

    // MARK: - Int Values
}

// MARK: - Auxiliary Methods

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    fn peek_byte(&mut self) -> Result<u8> {
        if let Some(byte) = self.peeked {
            return Ok(byte);
        }

        let byte = self.reader.read_one()?;

        self.peeked = Some(byte);

        Ok(byte)
    }

    fn pull_byte(&mut self) -> Result<u8> {
        let byte = match self.peeked {
            Some(byte) => byte,
            None => self.reader.read_one()?,
        };

        self.peeked = None;
        self.pos += 1;

        Ok(byte)
    }

    fn pull_bytes_into<'s>(&'s mut self, buf: &'s mut [u8]) -> Result<()> {
        let len = buf.len();

        if len == 0 {
            return Ok(());
        }

        if let Some(peeked) = self.peeked {
            self.reader.read_into(&mut buf[1..])?;
            buf[0] = peeked;
        } else {
            self.reader.read_into(buf)?;
        };

        self.peeked = None;
        self.pos += len;

        Ok(())
    }

    fn pull_bytes<'s>(
        &'s mut self,
        len: usize,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
        let bytes = self.reader.read(len, scratch)?;

        self.pos += bytes.len();

        Ok(bytes)
    }

    #[inline]
    pub fn peek_header<T>(&mut self) -> Result<T>
    where
        T: DecodeHeader,
    {
        let pos = self.pos;
        let byte = self.peek_byte()?;

        Self::decode_header(byte, Some(pos))
    }

    #[inline]
    fn pull_header<T>(&mut self) -> Result<T>
    where
        T: DecodeHeader,
    {
        let pos = self.pos;
        let byte = self.pull_byte()?;

        Self::decode_header(byte, Some(pos))
    }

    #[inline]
    fn decode_header<T>(byte: u8, pos: Option<usize>) -> Result<T>
    where
        T: DecodeHeader,
    {
        T::decode(byte).map_err(|exp| {
            Error::invalid_type(exp.unexpected.to_string(), exp.expected.to_string(), pos)
        })
    }

    #[inline]
    fn pull_len_bytes(&mut self, len_width: u8) -> Result<usize> {
        let pos = self.pos;

        self.pull_unsigned_extended_value(len_width)?
            .try_into()
            .map_err(|_| Error::number_out_of_range(Some(pos)))
    }
}

// MARK: - Tests

#[cfg(test)]
mod test {
    use crate::io::SliceReader;

    use super::*;

    #[test]
    fn new() {
        let bytes = SliceReader::new(&[1, 2, 3]);
        let decoder = Decoder::new(&bytes);
        assert_eq!(decoder.pos, 0);
    }

    #[test]
    fn pull_bytes_into() {
        let bytes = SliceReader::new(&[1, 2, 3]);
        let mut decoder = Decoder::new(bytes);
        assert_eq!(decoder.pos, 0);

        let mut buf = vec![];
        decoder.pull_bytes_into(&mut buf).unwrap();
        assert_eq!(buf, &[]);
        assert_eq!(decoder.pos, 0);

        let mut buf = vec![0];
        decoder.pull_bytes_into(&mut buf).unwrap();
        assert_eq!(buf, &[1]);
        assert_eq!(decoder.pos, 1);

        let mut buf = vec![0, 0];
        decoder.pull_bytes_into(&mut buf).unwrap();
        assert_eq!(buf, &[2, 3]);
        assert_eq!(decoder.pos, 3);

        let mut buf = vec![0, 0, 0];
        assert!(decoder.pull_bytes_into(&mut buf).is_err());
        assert_eq!(decoder.pos, 3);
    }
}
