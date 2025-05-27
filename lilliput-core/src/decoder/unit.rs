use crate::{error::Result, header::UnitHeader, marker::Marker, value::UnitValue};

use super::{Decoder, Read};

impl<'r, R> Decoder<R>
where
    R: Read<'r>,
{
    // MARK: - Value

    /// Decodes a unit value.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_unit(&mut self) -> Result<()> {
        self.decode_unit_header()?;

        Ok(())
    }

    /// Decodes a unit value, as a `UnitValue`.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_unit_value(&mut self) -> Result<UnitValue> {
        self.decode_unit().map(From::from)
    }

    // MARK: - Header

    /// Decodes a unit value's header.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_unit_header(&mut self) -> Result<UnitHeader> {
        #[allow(unused_variables)]
        let byte = self.pull_byte_expecting(Marker::Unit)?;

        #[cfg(feature = "tracing")]
        tracing::debug!(byte = crate::binary::fmt_byte(byte));

        Ok(UnitHeader)
    }

    // MARK: - Skip

    /// Skips the unit value for a given `header`.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn skip_unit_value_of(&mut self, header: UnitHeader) -> Result<()> {
        let _ = header;

        Ok(())
    }

    // MARK: - Body

    /// Decodes unit value for a given `header`, as a `UnitValue`.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_unit_value_of(&mut self, header: UnitHeader) -> Result<UnitValue> {
        let _ = header;

        Ok(UnitValue)
    }
}
