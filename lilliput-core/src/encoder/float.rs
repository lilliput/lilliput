use num_traits::{float::FloatCore, ToBytes};

use crate::{
    header::{EncodeHeader, FloatHeader},
    value::FloatValue,
    Profile,
};

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
        let profile = self.inner.profile;

        let width = match profile {
            Profile::Weak | Profile::None => N,
        };

        // Push the value's header:
        let header = FloatHeader::new(width);
        self.inner.push_byte(header.encode())?;

        // Push the value's actual bytes:
        match profile {
            Profile::Weak => self.push_value_bytes_variable(value, width),
            Profile::None => self.push_value_bytes_fixed(value, width),
        }
    }

    pub(super) fn encode_float_value(&mut self, value: &FloatValue) -> Result<(), EncoderError> {
        match *value {
            FloatValue::F32(value) => self.encode_float(value),
            FloatValue::F64(value) => self.encode_float(value),
        }
    }

    fn push_value_bytes_variable<T, const N: usize>(
        &mut self,
        value: T,
        width: usize,
    ) -> Result<(), EncoderError>
    where
        T: FloatCore + ToBytes<Bytes = [u8; N]>,
    {
        // FIXME: replace with proper variable encoding logic!
        self.push_value_bytes_fixed(value, width)
    }

    fn push_value_bytes_fixed<T, const N: usize>(
        &mut self,
        value: T,
        width: usize,
    ) -> Result<(), EncoderError>
    where
        T: FloatCore + ToBytes<Bytes = [u8; N]>,
    {
        let bytes = value.to_be_bytes();
        assert_eq!(bytes.len(), width);
        self.inner.push_bytes(&value.to_be_bytes())
    }
}
