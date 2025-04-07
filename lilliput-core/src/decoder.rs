use crate::{
    error::{Error, Result},
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
        self.peek_byte().map(Marker::detect)
    }

    pub fn decode_value(&mut self) -> Result<Value> {
        match self.peek_marker()? {
            Marker::Int => self.decode_int_value().map(From::from),
            Marker::String => self.decode_string_value().map(From::from),
            Marker::Seq => self.decode_seq_value().map(From::from),
            Marker::Map => self.decode_map_value().map(From::from),
            Marker::Float => self.decode_float_value().map(From::from),
            Marker::Bytes => self.decode_bytes_value().map(From::from),
            Marker::Bool => self.decode_bool_value().map(From::from),
            Marker::Null => self.decode_null_value().map(From::from),
            Marker::Reserved => unimplemented!(),
        }
    }
}

// MARK: - Auxiliary Methods

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    #[inline]
    fn peek_byte(&mut self) -> Result<u8> {
        if let Some(byte) = self.peeked {
            return Ok(byte);
        }

        let byte = self.reader.read_one()?;

        self.peeked = Some(byte);

        Ok(byte)
    }

    #[inline]
    fn pull_byte_expecting(&mut self, marker: Marker) -> Result<u8> {
        let pos = self.pos;

        let byte = self.pull_byte()?;

        marker.validate(byte).map_err(|exp| {
            Error::invalid_type(
                exp.unexpected.to_string(),
                exp.expected.to_string(),
                Some(pos),
            )
        })?;

        Ok(byte)
    }

    #[inline]
    fn pull_byte(&mut self) -> Result<u8> {
        let byte = match self.peeked {
            Some(byte) => byte,
            None => self.reader.read_one()?,
        };

        self.peeked = None;
        self.pos += 1;

        Ok(byte)
    }

    #[inline]
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

    #[inline]
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
    fn pull_len_bytes(&mut self, width: u8) -> Result<usize> {
        let pos = self.pos;

        const MAX_WIDTH: usize = 8;
        let mut padded_be_bytes: [u8; MAX_WIDTH] = [0b0; MAX_WIDTH];
        self.pull_bytes_into(&mut padded_be_bytes[(MAX_WIDTH - (width as usize))..])?;

        u64::from_be_bytes(padded_be_bytes)
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
        assert_eq!(buf.len(), 0);
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
