use crate::value::{NullValue, ValueType};

use super::{Decoder, DecoderError};

#[derive(Debug)]
pub struct NullDecoder<'a, 'de> {
    inner: &'de mut Decoder<'a>,
}

impl<'a, 'de> NullDecoder<'a, 'de> {
    pub(super) fn with(inner: &'de mut Decoder<'a>) -> Self {
        Self { inner }
    }

    pub(super) fn decode_null(&mut self) -> Result<(), DecoderError> {
        let _byte = self.inner.pull_byte_expecting_type(ValueType::Null)?;

        Ok(())
    }

    pub(super) fn decode_null_value(&mut self) -> Result<NullValue, DecoderError> {
        self.decode_null()?;

        Ok(NullValue)
    }
}
