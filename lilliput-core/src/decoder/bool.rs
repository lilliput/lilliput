use crate::{error::Result, header::BoolHeader, io::Read, marker::Marker, value::BoolValue};

use super::Decoder;

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    // MARK: - Value

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_bool(&mut self) -> Result<bool>
    where
        R: Read<'de>,
    {
        let header: BoolHeader = self.decode_bool_header()?;

        Ok(header.value())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_bool_value(&mut self) -> Result<BoolValue> {
        self.decode_bool().map(From::from)
    }

    // MARK: - Header

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_bool_header(&mut self) -> Result<BoolHeader> {
        let byte = self.pull_byte_expecting(Marker::Bool)?;

        let value = (byte & BoolHeader::VALUE_BIT) != 0b0;

        #[cfg(feature = "tracing")]
        tracing::debug!(byte = crate::binary::fmt_byte(byte), value = value);

        Ok(BoolHeader::new(value))
    }

    // MARK: - Body

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_bool_value_of(&mut self, header: BoolHeader) -> Result<BoolValue> {
        self.decode_bool_of(header).map(From::from)
    }

    // MARK: - Private

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    fn decode_bool_of(&mut self, header: BoolHeader) -> Result<bool> {
        Ok(header.value())
    }
}
