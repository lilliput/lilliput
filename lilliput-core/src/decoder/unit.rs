use crate::{error::Result, header::UnitHeader, marker::Marker, value::UnitValue};

use super::{Decoder, Read};

impl<'r, R> Decoder<R>
where
    R: Read<'r>,
{
    // MARK: - Value

    pub fn decode_unit(&mut self) -> Result<()> {
        self.decode_unit_header()?;

        Ok(())
    }

    pub fn decode_unit_value(&mut self) -> Result<UnitValue> {
        self.decode_unit().map(From::from)
    }

    // MARK: - Header

    pub fn decode_unit_header(&mut self) -> Result<UnitHeader> {
        let _ = self.pull_byte_expecting(Marker::Unit)?;

        Ok(UnitHeader)
    }

    // MARK: - Body

    pub fn decode_unit_value_of(&mut self, header: UnitHeader) -> Result<UnitValue> {
        let _ = header;

        Ok(UnitValue)
    }
}
