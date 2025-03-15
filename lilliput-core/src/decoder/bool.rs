use crate::value::{BoolValue, ValueType};

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
        let byte = self.inner.pull_byte_expecting_type(ValueType::Bool)?;

        let value = byte & BoolValue::VALUE_BIT != 0b0;

        Ok(value)
    }

    pub(super) fn decode_bool_value(&mut self) -> Result<BoolValue, DecoderError> {
        self.decode_bool().map(From::from)
    }
}
