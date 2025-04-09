use crate::{error::Result, header::NullHeader, io::Write, value::NullValue};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    // MARK: - Value

    #[inline]
    pub fn encode_null(&mut self) -> Result<()> {
        let header = self.header_for_null();
        self.encode_null_header(&header)
    }

    #[inline]
    pub fn encode_null_value(&mut self, value: &NullValue) -> Result<()> {
        let _ = value;
        self.encode_null()
    }

    // MARK: - Header

    #[inline]
    pub fn encode_null_header(&mut self, _header: &NullHeader) -> Result<()> {
        let byte = NullHeader::TYPE_BITS;

        self.push_byte(byte)
    }

    pub fn header_for_null(&self) -> NullHeader {
        NullHeader
    }
}
