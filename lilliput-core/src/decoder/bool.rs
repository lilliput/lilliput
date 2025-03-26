use crate::{error::Result, header::BoolHeader, io::Read, value::BoolValue};

use super::Decoder;

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    pub fn decode_bool(&mut self) -> Result<bool>
    where
        R: Read<'de>,
    {
        let header: BoolHeader = self.pull_header()?;

        Ok(header.value())
    }

    pub fn decode_bool_value(&mut self) -> Result<BoolValue> {
        self.decode_bool().map(From::from)
    }
}
