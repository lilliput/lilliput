use crate::{error::Result, header::NullHeader, io::Write, value::NullValue};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    // MARK: - Value

    /// Encodes a null value.
    #[inline]
    pub fn encode_null(&mut self) -> Result<()> {
        let header = self.header_for_null();
        self.encode_null_header(&header)
    }

    /// Encodes a null value, as a `NullValue`.
    #[inline]
    pub fn encode_null_value(&mut self, value: &NullValue) -> Result<()> {
        let _ = value;
        self.encode_null()
    }

    // MARK: - Header

    /// Encodes a null value's header.
    #[inline]
    pub fn encode_null_header(&mut self, header: &NullHeader) -> Result<()> {
        let _ = header;

        let byte = NullHeader::TYPE_BITS;

        #[cfg(feature = "tracing")]
        tracing::debug!(byte = crate::binary::fmt_byte(byte));

        self.push_byte(byte)
    }

    /// Creates a header for a null value.
    pub fn header_for_null(&self) -> NullHeader {
        NullHeader
    }
}
