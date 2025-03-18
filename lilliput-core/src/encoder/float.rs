use num_traits::{float::FloatCore, ToBytes};

use crate::{
    error::Result,
    header::{EncodeHeader, FloatHeader},
    io::Write,
    value::FloatValue,
    Profile,
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
        let profile = self.profile;

        let width = match profile {
            Profile::Weak | Profile::None => N,
        };

        // Push the value's header:
        let header = FloatHeader::new(width);
        self.push_bytes(&[header.encode()])?;

        // Push the value's actual bytes:
        match profile {
            Profile::Weak => self.push_value_bytes_variable(value, width),
            Profile::None => self.push_value_bytes_fixed(value, width),
        }
    }

    fn push_value_bytes_variable<T, const N: usize>(&mut self, value: T, width: usize) -> Result<()>
    where
        T: FloatCore + ToBytes<Bytes = [u8; N]>,
    {
        // FIXME: replace with proper variable encoding logic!
        self.push_value_bytes_fixed(value, width)
    }

    fn push_value_bytes_fixed<T, const N: usize>(&mut self, value: T, width: usize) -> Result<()>
    where
        T: FloatCore + ToBytes<Bytes = [u8; N]>,
    {
        let bytes = value.to_be_bytes();
        assert_eq!(bytes.len(), width);
        self.push_bytes(&value.to_be_bytes())
    }
}
