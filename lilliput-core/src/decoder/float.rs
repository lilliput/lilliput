use crate::{
    num::{FromFloat, IntoFloat as _},
    value::{FloatValue, ValueType},
};

use super::{Decoder, DecoderError};

#[derive(Debug)]
pub struct FloatDecoder<'a, 'de> {
    inner: &'de mut Decoder<'a>,
}

impl<'a, 'de> FloatDecoder<'a, 'de> {
    pub(super) fn with(inner: &'de mut Decoder<'a>) -> Self {
        Self { inner }
    }

    pub(super) fn decode_float<T>(&mut self) -> Result<T, DecoderError>
    where
        T: FromFloat<f32> + FromFloat<f64>,
    {
        let byte = self.inner.pull_byte_expecting_type(ValueType::Float)?;

        let width = (byte & FloatValue::WIDTH_BITS) as usize + 1;

        match width {
            4 => {
                let mut bytes: [u8; 4] = [0b0; 4];
                bytes.copy_from_slice(self.inner.pull_bytes(width)?);

                Ok(f32::from_be_bytes(bytes).into_float())
            }
            8 => {
                let mut bytes: [u8; 8] = [0b0; 8];
                bytes.copy_from_slice(self.inner.pull_bytes(width)?);

                Ok(f64::from_be_bytes(bytes).into_float())
            }
            _ => Err(DecoderError::IncompatibleProfile),
        }
    }

    pub(super) fn decode_float_value(&mut self) -> Result<FloatValue, DecoderError> {
        let byte = self.inner.peek_byte_expecting_type(ValueType::Float)?;

        let width = (byte & FloatValue::WIDTH_BITS) as usize + 1;

        match width {
            4 => self.decode_float().map(FloatValue::F32),
            8 => self.decode_float().map(FloatValue::F64),
            _ => Err(DecoderError::IncompatibleProfile),
        }
    }
}
