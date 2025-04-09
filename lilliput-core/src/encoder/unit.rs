use crate::{error::Result, header::UnitHeader, io::Write, value::UnitValue};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    // MARK: - Value

    #[inline]
    pub fn encode_unit(&mut self) -> Result<()> {
        let header = self.header_for_unit();
        self.encode_unit_header(&header)
    }

    #[inline]
    pub fn encode_unit_value(&mut self, value: &UnitValue) -> Result<()> {
        let _ = value;
        self.encode_unit()
    }

    // MARK: - Header

    #[inline]
    pub fn encode_unit_header(&mut self, _header: &UnitHeader) -> Result<()> {
        let byte = UnitHeader::TYPE_BITS;

        self.push_byte(byte)
    }

    pub fn header_for_unit(&self) -> UnitHeader {
        UnitHeader
    }
}
