use crate::{
    header::{BoolHeader, EncodeHeader as _},
    value::BoolValue,
};

use super::{Encoder, EncoderError};

#[derive(Debug)]
pub(super) struct BoolEncoder<'en> {
    inner: &'en mut Encoder,
}

impl<'en> BoolEncoder<'en> {
    pub(super) fn with(inner: &'en mut Encoder) -> Self {
        Self { inner }
    }

    pub(super) fn encode_bool(&mut self, value: bool) -> Result<(), EncoderError> {
        let header = BoolHeader::new(value);

        self.inner.push_byte(header.encode())?;

        Ok(())
    }

    pub(super) fn encode_bool_value(&mut self, value: &BoolValue) -> Result<(), EncoderError> {
        self.encode_bool(value.0)
    }
}
