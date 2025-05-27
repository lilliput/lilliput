use crate::{error::Result, header::BoolHeader, io::Read, marker::Marker, value::BoolValue};

use super::Decoder;

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    // MARK: - Value

    /// Decodes a boolean value.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_bool(&mut self) -> Result<bool>
    where
        R: Read<'de>,
    {
        let header: BoolHeader = self.decode_bool_header()?;

        self.decode_bool_of(header)
    }

    /// Decodes a boolean value, as a `BoolValue`.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_bool_value(&mut self) -> Result<BoolValue> {
        self.decode_bool().map(From::from)
    }

    // MARK: - Header

    /// Decodes a boolean value's header.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_bool_header(&mut self) -> Result<BoolHeader> {
        let byte = self.pull_byte_expecting(Marker::Bool)?;

        let value = (byte & BoolHeader::VALUE_BIT) != 0b0;

        #[cfg(feature = "tracing")]
        tracing::debug!(byte = crate::binary::fmt_byte(byte), value = value);

        Ok(BoolHeader::new(value))
    }

    // MARK: - Skip

    /// Skips the boolean value for a given `header`.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn skip_bool_value_of(&mut self, header: BoolHeader) -> Result<()>
    where
        R: Read<'de>,
    {
        let _ = header;
        self.reader.skip_one()
    }

    // MARK: - Body

    /// Decodes boolean value for a given `header`, as a `BoolValue`.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_bool_value_of(&mut self, header: BoolHeader) -> Result<BoolValue> {
        self.decode_bool_of(header).map(From::from)
    }

    // MARK: - Private

    /// Decodes boolean value for a given `header`.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    fn decode_bool_of(&mut self, header: BoolHeader) -> Result<bool> {
        Ok(header.value())
    }
}
