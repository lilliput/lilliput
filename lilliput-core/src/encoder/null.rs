use crate::{error::Result, header::NullHeader, io::Write, value::NullValue};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    #[inline]
    pub fn encode_null(&mut self) -> Result<()> {
        self.push_byte(0b00000000)
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

    pub fn header_for_null(&self) -> NullHeader {
        NullHeader
    }
}
