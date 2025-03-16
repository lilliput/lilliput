use crate::{
    header::{EncodeHeader, NullHeader},
    value::NullValue,
};

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
        let header = NullHeader;

        self.inner.push_byte(header.encode())?;

        Ok(())
    }

    pub(super) fn encode_null_value(&mut self, value: &NullValue) -> Result<(), EncoderError> {
        let NullValue = value;
        self.encode_null()?;

        Ok(())
    }
}
