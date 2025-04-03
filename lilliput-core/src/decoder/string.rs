use std::ops::Range;

use crate::{
    error::{Error, Result},
    header::StringHeader,
    io::{Read, Reference},
    marker::Marker,
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
        let header = self.decode_string_header()?;

        scratch.clear();

        let start = self.pos;
        let bytes = self.pull_bytes(header.len(), scratch)?;
        let range = start..(start + bytes.len());

        Ok((bytes, range))
    }

    pub fn decode_string_header(&mut self) -> Result<StringHeader> {
        let header_byte = self.pull_byte_expecting(Marker::String)?;

        let is_compact = (header_byte & StringHeader::COMPACT_VARIANT_BIT) != 0b0;

        if is_compact {
            let len = header_byte & StringHeader::COMPACT_LEN_BITS;
            Ok(StringHeader::compact(len))
        } else {
            let len_width = 1 + (header_byte & StringHeader::EXTENDED_LEN_WIDTH_BITS);
            let len = self.pull_len_bytes(len_width)?;
            Ok(StringHeader::extended(len))
        }
    }
}
