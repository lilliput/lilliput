use crate::{error::Result, header::UnitHeader, io::Write, value::UnitValue};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    // MARK: - Value

    /// Encodes a unit value.
    #[inline]
    pub fn encode_unit(&mut self) -> Result<()> {
        let header = self.header_for_unit();
        self.encode_unit_header(&header)
    }

    /// Encodes a unit value, from a `UnitValue`.
    #[inline]
    pub fn encode_unit_value(&mut self, value: &UnitValue) -> Result<()> {
        let _ = value;
        self.encode_unit()
    }

    // MARK: - Header

    /// Encodes a unit value's header.
    #[inline]
    pub fn encode_unit_header(&mut self, header: &UnitHeader) -> Result<()> {
        let _ = header;

        let byte = UnitHeader::TYPE_BITS;

        #[cfg(feature = "tracing")]
        tracing::debug!(byte = crate::binary::fmt_byte(byte));

        self.push_byte(byte)
    }

    /// Creates a header for a unit value.
    pub fn header_for_unit(&self) -> UnitHeader {
        UnitHeader
    }
}
