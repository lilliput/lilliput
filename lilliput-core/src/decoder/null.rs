use crate::{header::NullHeader, value::NullValue};

use super::{BufRead, Decoder, DecoderError};

#[derive(Debug)]
pub struct NullDecoder<'de, R> {
    inner: &'de mut Decoder<R>,
}

impl<'de, R> NullDecoder<'de, R>
where
    R: BufRead,
{
    pub(super) fn with(inner: &'de mut Decoder<R>) -> Self {
        Self { inner }
    }

    pub(super) fn decode_null(&mut self) -> Result<(), DecoderError> {
        let _header: NullHeader = self.inner.pull_header()?;

        {
            // nothing left to decode for null values
        }

        Ok(())
    }

    pub(super) fn decode_null_value(&mut self) -> Result<NullValue, DecoderError> {
        self.decode_null()?;

        Ok(NullValue)
    }
}
