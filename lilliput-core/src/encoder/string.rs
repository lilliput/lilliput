use crate::value::StringValue;

use super::{Encoder, EncoderError};

#[derive(Debug)]
pub(super) struct StringEncoder<'en> {
    inner: &'en mut Encoder,
}

impl<'en> StringEncoder<'en> {
    pub(super) fn with(inner: &'en mut Encoder) -> Self {
        Self { inner }
    }

    pub(super) fn encode_string(&mut self, value: &str) -> Result<(), EncoderError> {
        let value: &str = value;

        // Push the value's metadata:
        let mut head_byte = StringValue::PREFIX_BIT;

        head_byte |= 8 - 1; // width, minus 1
        self.inner.push_byte(head_byte)?;

        // Push the value's length:
        let neck_bytes = value.len().to_be_bytes();
        self.inner.push_bytes(&neck_bytes)?;

        // Push the value's actual bytes:
        let tail_bytes = value.as_bytes();
        self.inner.push_bytes(tail_bytes)?;

        Ok(())
    }

    pub(super) fn encode_string_value(&mut self, value: &StringValue) -> Result<(), EncoderError> {
        self.encode_string(&value.0)?;

        Ok(())
    }
}
