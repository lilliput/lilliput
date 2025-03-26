use crate::{
    error::Result,
    header::{BoolHeader, EncodeHeader as _},
    io::Write,
    value::BoolValue,
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_bool(&mut self, value: bool) -> Result<()> {
        let header = BoolHeader::new(value);

        // Push the value's header:
        self.push_bytes(&[header.encode()])?;

        Ok(())
    }

    pub fn encode_bool_value(&mut self, value: &BoolValue) -> Result<()> {
        self.encode_bool(value.0)
    }
}
