use crate::{error::Result, header::BoolHeader, io::Read, marker::Marker, value::BoolValue};

use super::Decoder;

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    // MARK: - Value

    pub fn decode_bool(&mut self) -> Result<bool>
    where
        R: Read<'de>,
    {
        let header: BoolHeader = self.decode_bool_header()?;

        Ok(header.value())
    }

    pub fn decode_bool_value(&mut self) -> Result<BoolValue> {
        self.decode_bool().map(From::from)
    }

    // MARK: - Header

    pub fn decode_bool_header(&mut self) -> Result<BoolHeader> {
        let header_byte = self.pull_byte_expecting(Marker::Bool)?;

        let value = (header_byte & BoolHeader::VALUE_BIT) != 0b0;

        Ok(BoolHeader::new(value))
    }
}
