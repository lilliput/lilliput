use crate::{
    error::{Error, Result},
    header::FloatHeader,
    num::float::{FromFloat, IntoFloat as _},
    value::FloatValue,
};

use super::{Decoder, Read};

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    pub fn decode_f32(&mut self) -> Result<f32> {
        self.decode_float()
    }

    pub fn decode_f64(&mut self) -> Result<f64> {
        self.decode_float()
    }

    pub fn decode_float_value(&mut self) -> Result<FloatValue> {
        let header: FloatHeader = self.pull_header()?;
        let width = header.width();

        match width {
            4 => self.decode_float_value_bytes(width).map(FloatValue::F32),
            8 => self.decode_float_value_bytes(width).map(FloatValue::F64),
            _ => unreachable!(),
        }
    }

    fn decode_float<T>(&mut self) -> Result<T>
    where
        T: FromFloat<f32> + FromFloat<f64>,
    {
        let header: FloatHeader = self.pull_header()?;

        let width = header.width();

        self.decode_float_value_bytes(width)
    }

    #[inline(always)]
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
