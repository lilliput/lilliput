use crate::value::NullValue;

use super::{Encoder, EncoderError};

#[derive(Debug)]
pub(super) struct NullEncoder<'en> {
    inner: &'en mut Encoder,
}

impl<'en> NullEncoder<'en> {
    pub(super) fn with(inner: &'en mut Encoder) -> Self {
        Self { inner }
    }

    pub(super) fn encode_null(&mut self) -> Result<(), EncoderError> {
        let head_byte = NullValue::BIT_REPR;

        self.inner.push_byte(head_byte)?;

        Ok(())
    }

    pub(super) fn encode_null_value(&mut self, value: &NullValue) -> Result<(), EncoderError> {
        let NullValue = value;
        self.encode_null()?;

        Ok(())
    }
}
