use crate::{
    header::FloatHeader,
    num::{FromFloat, IntoFloat as _},
    value::FloatValue,
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
        let header: FloatHeader = self.inner.pull_header()?;
        let width = header.width();

        self.decode_value_bytes(width)
    }

    pub(super) fn decode_float_value(&mut self) -> Result<FloatValue, DecoderError> {
        let header: FloatHeader = self.inner.peek_header()?;
        let width = header.width();

        match width {
            1..=4 => self.decode_float().map(FloatValue::F32),
            5..=8 => self.decode_float().map(FloatValue::F64),
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn decode_value_bytes<T>(&mut self, width: usize) -> Result<T, DecoderError>
    where
        T: FromFloat<f32> + FromFloat<f64>,
    {
        let profile = self.inner.profile;

        match width {
            1..=3 => Err(DecoderError::Profile { profile }),
            4 => {
                let mut bytes: [u8; 4] = [0b0; 4];
                bytes.copy_from_slice(self.inner.pull_bytes(width)?);

                Ok(f32::from_be_bytes(bytes).into_float())
            }
            5..=7 => Err(DecoderError::Profile { profile }),
            8 => {
                let mut bytes: [u8; 8] = [0b0; 8];
                bytes.copy_from_slice(self.inner.pull_bytes(width)?);

                Ok(f64::from_be_bytes(bytes).into_float())
            }
            _ => unreachable!(),
        }
    }
}
