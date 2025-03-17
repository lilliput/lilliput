use crate::{
    header::{BoolHeader, EncodeHeader as _},
    io::Write,
    value::BoolValue,
};

use super::{Encoder, EncoderError};

#[derive(Debug)]
pub(super) struct BoolEncoder<'en, W> {
    inner: &'en mut Encoder<W>,
}

impl<'en, W> BoolEncoder<'en, W>
where
    W: Write,
{
    pub(super) fn with(inner: &'en mut Encoder<W>) -> Self {
        Self { inner }
    }

    pub(super) fn encode_bool(&mut self, value: bool) -> Result<(), EncoderError> {
        let header = BoolHeader::new(value);

        self.inner.push_bytes(&[header.encode()])?;

        Ok(())
    }

    pub(super) fn encode_bool_value(&mut self, value: &BoolValue) -> Result<(), EncoderError> {
        self.encode_bool(value.0)
    }
}
