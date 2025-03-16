use crate::{header::BoolHeader, value::BoolValue};

use super::{Decoder, DecoderError};

#[derive(Debug)]
pub struct BoolDecoder<'a, 'de> {
    inner: &'de mut Decoder<'a>,
}

impl<'a, 'de> BoolDecoder<'a, 'de> {
    pub(super) fn with(inner: &'de mut Decoder<'a>) -> Self {
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
