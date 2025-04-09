use crate::{error::Result, header::UnitHeader, marker::Marker, value::UnitValue};

use super::{Decoder, Read};

impl<'r, R> Decoder<R>
where
    R: Read<'r>,
{
    // MARK: - Value

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_unit(&mut self) -> Result<()> {
        self.decode_unit_header()?;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_unit_value(&mut self) -> Result<UnitValue> {
        self.decode_unit().map(From::from)
    }

    // MARK: - Header

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_unit_header(&mut self) -> Result<UnitHeader> {
        #[allow(unused_variables)]
        let byte = self.pull_byte_expecting(Marker::Unit)?;

        #[cfg(feature = "tracing")]
        tracing::debug!(byte = crate::binary::fmt_byte(byte));

        Ok(UnitHeader)
    }

    // MARK: - Body

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_unit_value_of(&mut self, header: UnitHeader) -> Result<UnitValue> {
        let _ = header;

        Ok(UnitValue)
    }
}
