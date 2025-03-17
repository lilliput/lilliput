use crate::{header::BoolHeader, value::BoolValue};

use super::{BufRead, Decoder, DecoderError};

#[derive(Debug)]
pub struct BoolDecoder<'de, R> {
    inner: &'de mut Decoder<R>,
}

impl<'de, R> BoolDecoder<'de, R>
where
    R: BufRead,
{
    pub(super) fn with(inner: &'de mut Decoder<R>) -> Self {
        Self { inner }
    }

    pub(super) fn decode_bool(&mut self) -> Result<bool, DecoderError> {
        let header: BoolHeader = self.inner.pull_header()?;

        {
            // nothing left to decode for bool values
        }

        Ok(header.value())
    }

    pub(super) fn decode_bool_value(&mut self) -> Result<BoolValue, DecoderError> {
        self.decode_bool().map(From::from)
    }
}
