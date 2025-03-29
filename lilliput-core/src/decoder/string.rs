use std::ops::Range;

use crate::{
    error::{Error, Result},
    header::{StringHeader, StringHeaderRepr},
    io::{Read, Reference},
    value::StringValue,
};

use super::Decoder;

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    pub fn decode_str<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, str>> {
        let (bytes, range) = self.decode_str_bytes_and_range(scratch)?;

        let str_ref = match bytes {
            Reference::Borrowed(bytes) => std::str::from_utf8(bytes).map(Reference::Borrowed),
            Reference::Copied(bytes) => std::str::from_utf8(bytes).map(Reference::Copied),
        }
        .map_err(|err| {
            let pos = range.start + err.valid_up_to() + 1;
            Error::utf8(err, Some(pos))
        })?;

        Ok(str_ref)
    }

    pub fn decode_str_bytes<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
        Ok(self.decode_str_bytes_and_range(scratch)?.0)
    }

    pub fn decode_string(&mut self) -> Result<String> {
        let (bytes_buf, range) = self.decode_string_bytes_buf_and_range()?;

        let string = String::from_utf8(bytes_buf).map_err(|err| {
            let err = err.utf8_error();
            let pos = range.start + err.valid_up_to() + 1;
            Error::utf8(err, Some(pos))
        })?;

        Ok(string)
    }

    pub fn decode_string_bytes_buf(&mut self) -> Result<Vec<u8>> {
        Ok(self.decode_string_bytes_buf_and_range()?.0)
    }

    pub fn decode_string_value(&mut self) -> Result<StringValue> {
        self.decode_string().map(From::from)
    }

    fn decode_string_bytes_buf_and_range(&mut self) -> Result<(Vec<u8>, Range<usize>)> {
        let mut buf = Vec::new();

        let (bytes, range) = self.decode_str_bytes_and_range(&mut buf)?;

        match bytes {
            Reference::Borrowed(slice) => {
                debug_assert_eq!(buf.len(), 0);
                buf.extend_from_slice(slice);
            }
            Reference::Copied(slice) => {
                debug_assert_eq!(slice.len(), buf.len());
            }
        }

        Ok((buf, range))
    }

    fn decode_str_bytes_and_range<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<(Reference<'de, 's, [u8]>, Range<usize>)> {
        let len = self.decode_str_header()?;

        scratch.clear();

        let start = self.pos;
        let bytes = self.pull_bytes(len, scratch)?;
        let range = start..(start + bytes.len());

        Ok((bytes, range))
    }

    fn decode_str_header(&mut self) -> Result<usize> {
        let header: StringHeader = self.pull_header()?;

        let len: usize = match header.repr() {
            StringHeaderRepr::Compact { len } => len.into(),
            StringHeaderRepr::Extended { len_width } => self.pull_len_bytes(len_width)?,
        };

        Ok(len)
    }
}
