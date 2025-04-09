use crate::{error::Result, header::FloatHeader, marker::Marker, value::FloatValue};

use super::{Decoder, Read};

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    // MARK: - Value

    pub fn decode_f32(&mut self) -> Result<f32> {
        let header = self.decode_float_header()?;
        Ok(self.decode_float_value_of(header)?.into())
    }

    pub fn decode_f64(&mut self) -> Result<f64> {
        let header = self.decode_float_header()?;
        Ok(self.decode_float_value_of(header)?.into())
    }

    pub fn decode_float_value(&mut self) -> Result<FloatValue> {
        let header = self.decode_float_header()?;

        self.decode_float_value_of(header)
    }

    // MARK: - Header

    pub fn decode_float_header(&mut self) -> Result<FloatHeader> {
        let byte = self.pull_byte_expecting(Marker::Float)?;

        let width = 1 + (byte & FloatHeader::VALUE_WIDTH_BITS);

        Ok(FloatHeader::new(width))
    }

    // MARK: - Body

    pub fn decode_float_value_of(&mut self, header: FloatHeader) -> Result<FloatValue> {
        match header.width() {
            1..=3 => unimplemented!(),
            4 => {
                let mut bytes: [u8; 4] = [0b0; 4];
                self.pull_bytes_into(&mut bytes)?;
                Ok(FloatValue::F32(f32::from_be_bytes(bytes)))
            }
            5..=7 => unimplemented!(),
            8 => {
                let mut bytes: [u8; 8] = [0b0; 8];
                self.pull_bytes_into(&mut bytes)?;
                Ok(FloatValue::F64(f64::from_be_bytes(bytes)))
            }
            _ => unreachable!(),
        }
    }
}
