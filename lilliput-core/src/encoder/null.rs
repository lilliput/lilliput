use crate::{
    error::Result,
    header::{EncodeHeader, NullHeader},
    io::Write,
    value::NullValue,
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_null(&mut self) -> Result<()> {
        let header = NullHeader;

        // Push the value's header:
        self.push_bytes(&[header.encode()])?;

        Ok(())
    }

    pub fn encode_null_value(&mut self, value: &NullValue) -> Result<()> {
        let NullValue(()) = value;
        self.encode_null()?;

        Ok(())
    }
}
