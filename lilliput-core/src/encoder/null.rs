use crate::{
    header::{EncodeHeader, NullHeader},
    io::Write,
    value::NullValue,
};

use super::{Encoder, EncoderError};

#[derive(Debug)]
pub(super) struct NullEncoder<'en, W> {
    inner: &'en mut Encoder<W>,
}

impl<'en, W> NullEncoder<'en, W>
where
    W: Write,
{
    pub(super) fn with(inner: &'en mut Encoder<W>) -> Self {
        Self { inner }
    }

    pub(super) fn encode_null(&mut self) -> Result<(), EncoderError> {
        let header = NullHeader;

        self.inner.push_bytes(&[header.encode()])?;

        Ok(())
    }

    pub(super) fn encode_null_value(&mut self, value: &NullValue) -> Result<(), EncoderError> {
        let NullValue = value;
        self.encode_null()?;

        Ok(())
    }
}
