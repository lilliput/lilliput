use num_traits::{float::FloatCore, ToBytes};

use crate::value::FloatValue;

use super::{Encoder, EncoderError};

#[derive(Debug)]
pub(super) struct FloatEncoder<'en> {
    inner: &'en mut Encoder,
}

impl<'en> FloatEncoder<'en> {
    pub(super) fn with(inner: &'en mut Encoder) -> Self {
        Self { inner }
    }

    pub(super) fn encode_float<T, const N: usize>(&mut self, value: T) -> Result<(), EncoderError>
    where
        T: FloatCore + ToBytes<Bytes = [u8; N]>,
    {
        // Push the value's metadata:
        let mut head_byte = FloatValue::PREFIX_BIT;

        head_byte |= (N as u8) - 1; // width of T, minus 1
        self.inner.push_byte(head_byte)?;

        // Push the value's actual bytes:
        let tail_bytes = value.to_be_bytes();
        self.inner.push_bytes(&tail_bytes)?;

        Ok(())
    }

    pub(super) fn encode_float_value(&mut self, value: &FloatValue) -> Result<(), EncoderError> {
        match *value {
            FloatValue::F32(value) => self.encode_float(value),
            FloatValue::F64(value) => self.encode_float(value),
        }
    }
}
