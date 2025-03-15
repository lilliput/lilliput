use crate::value::{SeqValue, Value};

use super::{Encoder, EncoderError};

#[derive(Debug)]
pub(super) struct SeqEncoder<'en> {
    inner: &'en mut Encoder,
}

impl<'en> SeqEncoder<'en> {
    pub(super) fn with(inner: &'en mut Encoder) -> Self {
        Self { inner }
    }

    pub(super) fn encode_seq(&mut self, value: &[Value]) -> Result<(), EncoderError> {
        self.encode_seq_start(value.len())?;

        for value in value {
            self.inner.encode_any(value)?;
        }

        self.encode_seq_end()
    }

    pub(super) fn encode_seq_value(&mut self, value: &SeqValue) -> Result<(), EncoderError> {
        self.encode_seq(&value.0)
    }

    pub(super) fn encode_seq_start(&mut self, len: usize) -> Result<(), EncoderError> {
        // Push the value's metadata:
        let mut head_byte = SeqValue::PREFIX_BIT;
        head_byte |= 8 - 1; // width, minus 1
        self.inner.push_byte(head_byte)?;

        // Push the value's length:
        let neck_bytes = len.to_be_bytes();
        self.inner.push_bytes(&neck_bytes)?;

        Ok(())
    }

    pub(super) fn encode_seq_end(&mut self) -> Result<(), EncoderError> {
        Ok(())
    }
}
