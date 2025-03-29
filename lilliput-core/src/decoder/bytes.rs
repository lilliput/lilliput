use crate::{
    error::Result,
    header::BytesHeader,
    io::{Read, Reference},
    value::BytesValue,
};

use super::Decoder;

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    pub fn decode_bytes<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
        let len = self.decode_bytes_header()?;

        self.pull_bytes(len, scratch)
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

    pub fn decode_bytes_header(&mut self) -> Result<usize> {
        let header: BytesHeader = self.pull_header()?;

        let len_width = header.len_width();
        self.pull_len_bytes(len_width)
    }

    pub fn decode_bytes_value(&mut self) -> Result<BytesValue> {
        self.decode_bytes_buf().map(From::from)
    }
}
