use crate::{error::Result, header::NullHeader, marker::Marker, value::NullValue};

use super::{Decoder, Read};

impl<'r, R> Decoder<R>
where
    R: Read<'r>,
{
    // MARK: - Value

    /// Decodes a null value.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_null(&mut self) -> Result<()> {
        self.decode_null_header()?;

        Ok(())
    }

    /// Decodes a null value, as a `NullValue`.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_null_value(&mut self) -> Result<NullValue> {
        self.decode_null().map(|_| NullValue)
    }

    // MARK: - Header

    /// Decodes a null value's header.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_null_header(&mut self) -> Result<NullHeader> {
        #[allow(unused_variables)]
        let byte = self.pull_byte_expecting(Marker::Null)?;

        #[cfg(feature = "tracing")]
        tracing::debug!(byte = crate::binary::fmt_byte(byte),);

        Ok(NullHeader)
    }

    // MARK: - Skip

    /// Skips the null value for a given `header`.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn skip_null_value_of(&mut self, header: NullHeader) -> Result<()> {
        let _ = header;

        Ok(())
    }

    // MARK: - Body

    /// Decodes null value for a given `header`, as a `NullValue`.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_null_value_of(&mut self, header: NullHeader) -> Result<NullValue> {
        let _ = header;

        Ok(NullValue)
    }
}
