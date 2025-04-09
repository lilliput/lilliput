use crate::{binary, error::Result, header::BoolHeader, io::Write, value::BoolValue};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    // MARK: - Value

    #[inline]
    pub fn encode_bool(&mut self, value: bool) -> Result<()> {
        let header = self.header_for_bool(value);
        self.encode_bool_header(&header)
    }

    #[inline]
    pub fn encode_bool_value(&mut self, value: &BoolValue) -> Result<()> {
        self.encode_bool(value.0)
    }

    // MARK: - Header

    #[inline]
    pub fn encode_bool_header(&mut self, header: &BoolHeader) -> Result<()> {
        let mut byte = BoolHeader::TYPE_BITS;

        byte |= binary::bits_if(BoolHeader::VALUE_BIT, header.value());

        self.push_byte(byte)
    }

    #[inline]
    pub fn header_for_bool(&self, value: bool) -> BoolHeader {
        BoolHeader::new(value)
    }
}
