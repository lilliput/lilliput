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
        self.decode_bool_headed_by(header)
    }

    pub fn decode_bool_value(&mut self) -> Result<BoolValue> {
        let header: BoolHeader = self.pull_header()?;
        self.decode_bool_value_headed_by(header)
    }

    pub(super) fn decode_bool_value_headed_by(&mut self, header: BoolHeader) -> Result<BoolValue> {
        self.decode_bool_headed_by(header).map(From::from)
    }

    fn decode_bool_headed_by(&mut self, header: BoolHeader) -> Result<bool>
    where
        R: Read<'de>,
    {
        Ok(header.value())
    }
}
