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
    // MARK: - Value

    pub fn decode_str<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, str>> {
        let header = self.decode_string_header()?;
        self.decode_str_of(header, scratch)
    }

    pub fn decode_str_bytes<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
        let header = self.decode_string_header()?;
        self.decode_str_bytes_of(header, scratch)
    }

    pub fn decode_string(&mut self) -> Result<String> {
        let header = self.decode_string_header()?;
        self.decode_string_of(header)
    }

    pub fn decode_string_bytes_buf(&mut self) -> Result<Vec<u8>> {
        let header = self.decode_string_header()?;
        self.decode_string_bytes_buf_of(header)
    }

    pub fn decode_string_value(&mut self) -> Result<StringValue> {
        let header = self.decode_string_header()?;
        self.decode_string_value_of(header)
    }

    // MARK: - Header

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

    // MARK: - Body

    pub fn decode_string_value_of(&mut self, header: StringHeader) -> Result<StringValue> {
        self.decode_string_of(header).map(From::from)
    }

    // MARK: - Private

    fn decode_str_of<'s>(
        &'s mut self,
        header: StringHeader,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, str>> {
        let (bytes, range) = self.decode_str_bytes_and_range_of(header, scratch)?;

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

    fn decode_str_bytes_of<'s>(
        &'s mut self,
        header: StringHeader,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
        Ok(self.decode_str_bytes_and_range_of(header, scratch)?.0)
    }

    fn decode_string_of(&mut self, header: StringHeader) -> Result<String> {
        let (bytes_buf, range) = self.decode_string_bytes_buf_and_range_of(header)?;

        let string = String::from_utf8(bytes_buf).map_err(|err| {
            let err = err.utf8_error();
            let pos = range.start + err.valid_up_to() + 1;
            Error::utf8(err, Some(pos))
        })?;

        Ok(string)
    }

    fn decode_string_bytes_buf_of(&mut self, header: StringHeader) -> Result<Vec<u8>> {
        Ok(self.decode_string_bytes_buf_and_range_of(header)?.0)
    }

    fn decode_string_bytes_buf_and_range_of(
        &mut self,
        header: StringHeader,
    ) -> Result<(Vec<u8>, Range<usize>)> {
        let mut buf = Vec::new();

        let (bytes, range) = self.decode_str_bytes_and_range_of(header, &mut buf)?;

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

    fn decode_str_bytes_and_range_of<'s>(
        &'s mut self,
        header: StringHeader,
        scratch: &'s mut Vec<u8>,
    ) -> Result<(Reference<'de, 's, [u8]>, Range<usize>)> {
        scratch.clear();

        let start = self.pos;
        let bytes = self.pull_bytes(header.len(), scratch)?;
        let range = start..(start + bytes.len());

        Ok((bytes, range))
    }
}
