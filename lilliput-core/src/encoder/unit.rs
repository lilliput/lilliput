use crate::{error::Result, header::UnitHeader, io::Write, value::UnitValue};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    #[inline]
    pub fn encode_unit(&mut self) -> Result<()> {
        self.push_byte(0b00000001)
    }

    #[inline]
    pub fn encode_unit_value(&mut self, value: &UnitValue) -> Result<()> {
        let UnitValue(()) = value;
        self.encode_unit()
    }

    // MARK: - Header

    #[inline]
    pub fn encode_unit_header(&mut self, header: &UnitHeader) -> Result<()> {
        let _ = header;
        self.encode_unit()
    }

    pub fn header_for_unit(&self) -> UnitHeader {
        UnitHeader
    }
}
