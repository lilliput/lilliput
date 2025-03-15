use crate::value::BytesValue;

use super::{Encoder, EncoderError};

#[derive(Debug)]
pub(super) struct BytesEncoder<'en> {
    inner: &'en mut Encoder,
}

impl<'en> BytesEncoder<'en> {
    pub(super) fn with(inner: &'en mut Encoder) -> Self {
        Self { inner }
    }

    pub(super) fn encode_bytes(&mut self, value: &[u8]) -> Result<(), EncoderError> {
        // Push the value's metadata:
        let mut head_byte = BytesValue::PREFIX_BIT;
        head_byte |= 3; // width exponent of usize (2 ^ 3 = 8)
        self.inner.push_byte(head_byte)?;

        // Push the value's length:
        let neck_bytes = value.len().to_be_bytes();
        self.inner.push_bytes(&neck_bytes)?;

        // Push the value's actual bytes:
        let tail_bytes = value;
        self.inner.push_bytes(tail_bytes)?;

        Ok(())
    }

    pub(super) fn encode_bytes_value(&mut self, value: &BytesValue) -> Result<(), EncoderError> {
        self.encode_bytes(&value.0)
    }
}
