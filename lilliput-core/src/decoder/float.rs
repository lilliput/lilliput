use crate::{
    error::{Error, Result},
    header::FloatHeader,
    marker::Marker,
    num::float::{FromFloat, IntoFloat as _},
    value::FloatValue,
};

use super::{Decoder, Read};

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    pub fn decode_f32(&mut self) -> Result<f32> {
        match self.decode_float_value()? {
            FloatValue::F32(value) => Ok(value),
            FloatValue::F64(value) => {
                // FIXME: add a strictness config?
                Ok(value as f32)
            }
        }
    }

    pub fn decode_f64(&mut self) -> Result<f64> {
        match self.decode_float_value()? {
            FloatValue::F32(value) => {
                // FIXME: add a strictness config?
                Ok(value as f64)
            }
            FloatValue::F64(value) => Ok(value),
        }
    }

    pub fn decode_float_value(&mut self) -> Result<FloatValue> {
        self.decode_float_header().map(|header| header.value())
    }

    pub fn decode_float_header(&mut self) -> Result<FloatHeader> {
        let header_byte = self.pull_byte_expecting(Marker::Float)?;

        let width = 1 + (header_byte & FloatHeader::VALUE_WIDTH_BITS);

        let value = match width {
            width @ 4 => self.decode_float_value_bytes(width).map(FloatValue::F32),
            width @ 8 => self.decode_float_value_bytes(width).map(FloatValue::F64),
            // FIXME: add support for var-float:
            _ => unreachable!(),
        }?;

        Ok(FloatHeader::new(value))
    }

    #[inline]
    fn decode_float_value_bytes<T>(&mut self, width: u8) -> Result<T>
    where
        T: FromFloat<f32> + FromFloat<f64>,
    {
        let pos = self.pos;

        match width {
            1..=3 => Err(Error::uncategorized("unsupported profile", Some(pos))),
            4 => {
                let mut bytes: [u8; 4] = [0b0; 4];
                self.pull_bytes_into(&mut bytes)?;

                Ok(f32::from_be_bytes(bytes).into_float())
            }
            5..=7 => Err(Error::uncategorized("unsupported profile", Some(pos))),
            8 => {
                let mut bytes: [u8; 8] = [0b0; 8];
                self.pull_bytes_into(&mut bytes)?;

                Ok(f64::from_be_bytes(bytes).into_float())
            }
            _ => unreachable!(),
        }
    }
}
