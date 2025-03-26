use num_traits::{float::FloatCore, ToBytes};

use crate::{
    error::Result,
    header::{EncodeHeader, FloatHeader},
    io::Write,
    value::FloatValue,
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_f32(&mut self, value: f32) -> Result<()> {
        self.encode_float(value)
    }

    pub fn encode_f64(&mut self, value: f64) -> Result<()> {
        self.encode_float(value)
    }

    pub fn encode_float_value(&mut self, value: &FloatValue) -> Result<()> {
        match *value {
            FloatValue::F32(value) => self.encode_float(value),
            FloatValue::F64(value) => self.encode_float(value),
        }
    }

    fn encode_float<T, const N: usize>(&mut self, value: T) -> Result<()>
    where
        T: FloatCore + ToBytes<Bytes = [u8; N]>,
    {
        let width = N as u8;

        let header = FloatHeader::new(width);

        // Push the value's header:
        self.push_bytes(&[header.encode()])?;

        // Push the value's actual bytes:
        let be_bytes = value.to_be_bytes();
        debug_assert_eq!(be_bytes.len(), width as usize);
        self.push_bytes(&be_bytes)
    }
}
