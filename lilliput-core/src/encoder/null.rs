use crate::{error::Result, header::NullHeader, io::Write, value::NullValue};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    #[inline]
    pub fn encode_null(&mut self) -> Result<()> {
        let header_byte = NullHeader::TYPE_BITS;

        self.push_byte(header_byte)
    }

    #[inline]
    pub fn encode_null_value(&mut self, value: &NullValue) -> Result<()> {
        let NullValue(()) = value;
        self.encode_null()
    }

    #[inline]
    pub fn encode_null_header(&mut self, header: &NullHeader) -> Result<()> {
        let _ = header;
        self.encode_null()
    }
}
