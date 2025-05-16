use lilliput_float::{FpExtend as _, FpFromBeBytes as _, F16, F24, F32, F40, F48, F56, F64, F8};

use crate::{error::Result, header::FloatHeader, marker::Marker, value::FloatValue};

use super::{Decoder, Read};

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    // MARK: - Value

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_f32(&mut self) -> Result<f32> {
        let header = self.decode_float_header()?;
        Ok(self.decode_float_value_of(header)?.into())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_f64(&mut self) -> Result<f64> {
        let header = self.decode_float_header()?;
        Ok(self.decode_float_value_of(header)?.into())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_float_value(&mut self) -> Result<FloatValue> {
        let header = self.decode_float_header()?;

        self.decode_float_value_of(header)
    }

    // MARK: - Header

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_float_header(&mut self) -> Result<FloatHeader> {
        let byte = self.pull_byte_expecting(Marker::Float)?;

        let width = 1 + (byte & FloatHeader::VALUE_WIDTH_BITS);

        #[cfg(feature = "tracing")]
        tracing::debug!(byte = crate::binary::fmt_byte(byte), width = width);

        Ok(FloatHeader::new(width))
    }

    // MARK: - Skip

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn skip_float_value_of(&mut self, header: FloatHeader) -> Result<()> {
        self.reader.skip(header.width().into())
    }

    // MARK: - Body

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_float_value_of(&mut self, header: FloatHeader) -> Result<FloatValue> {
        match header.width() {
            1 => {
                let mut bytes: [u8; 1] = [0b0; 1];
                self.pull_bytes_into(&mut bytes)?;
                let packed = F8::from_be_bytes(bytes);
                let unpacked: F32 = packed.extend();
                Ok(FloatValue::F32(unpacked.into()))
            }
            2 => {
                let mut bytes: [u8; 2] = [0b0; 2];
                self.pull_bytes_into(&mut bytes)?;
                let packed = F16::from_be_bytes(bytes);
                let unpacked: F32 = packed.extend();
                Ok(FloatValue::F32(unpacked.into()))
            }
            3 => {
                let mut bytes: [u8; 3] = [0b0; 3];
                self.pull_bytes_into(&mut bytes)?;
                let packed = F24::from_be_bytes(bytes);
                let unpacked: F32 = packed.extend();
                Ok(FloatValue::F32(unpacked.into()))
            }
            4 => {
                let mut bytes: [u8; 4] = [0b0; 4];
                self.pull_bytes_into(&mut bytes)?;
                let value = F32::from_be_bytes(bytes);
                Ok(FloatValue::F32(value.into()))
            }
            5 => {
                let mut bytes: [u8; 5] = [0b0; 5];
                self.pull_bytes_into(&mut bytes)?;
                let packed = F40::from_be_bytes(bytes);
                let unpacked: F64 = packed.extend();
                Ok(FloatValue::F64(unpacked.into()))
            }
            6 => {
                let mut bytes: [u8; 6] = [0b0; 6];
                self.pull_bytes_into(&mut bytes)?;
                let packed = F48::from_be_bytes(bytes);
                let unpacked: F64 = packed.extend();
                Ok(FloatValue::F64(unpacked.into()))
            }
            7 => {
                let mut bytes: [u8; 7] = [0b0; 7];
                self.pull_bytes_into(&mut bytes)?;
                let packed = F56::from_be_bytes(bytes);
                let unpacked: F64 = packed.extend();
                Ok(FloatValue::F64(unpacked.into()))
            }
            8 => {
                let mut bytes: [u8; 8] = [0b0; 8];
                self.pull_bytes_into(&mut bytes)?;
                let value = F64::from_be_bytes(bytes);
                Ok(FloatValue::F64(value.into()))
            }
            _ => unreachable!(),
        }
    }
}
