use crate::{
    error::Result,
    header::BytesHeader,
    io::{Read, Reference},
    marker::Marker,
    value::BytesValue,
};

use super::Decoder;

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    // MARK: - Value

    pub fn decode_bytes<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
        let header = self.decode_bytes_header()?;

        self.pull_bytes(header.len(), scratch)
    }

    pub fn decode_bytes_buf(&mut self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        match self.decode_bytes(&mut buf)? {
            Reference::Borrowed(slice) => {
                debug_assert_eq!(buf.len(), 0);
                buf.extend_from_slice(slice);
            }
            Reference::Copied(slice) => {
                debug_assert_eq!(slice.len(), buf.len());
            }
        }

        Ok(buf)
    }

    pub fn decode_bytes_value(&mut self) -> Result<BytesValue> {
        self.decode_bytes_buf().map(From::from)
    }

    // MARK: - Header

    pub fn decode_bytes_header(&mut self) -> Result<BytesHeader> {
        let header_byte = self.pull_byte_expecting(Marker::Bytes)?;

        let len_width_exponent = header_byte & BytesHeader::LEN_WIDTH_EXPONENT_BITS;

        let len_width: u8 = 1 << len_width_exponent;
        let len = self.pull_len_bytes(len_width)?;

        Ok(BytesHeader::new(len))
    }
}
