use crate::{error::Result, header::NullHeader, marker::Marker, value::NullValue};

use super::{Decoder, Read};

impl<'r, R> Decoder<R>
where
    R: Read<'r>,
{
    // MARK: - Value

    pub fn decode_null(&mut self) -> Result<()> {
        self.decode_null_header()?;

        Ok(())
    }

    pub fn decode_null_value(&mut self) -> Result<NullValue> {
        self.decode_null().map(From::from)
    }

    // MARK: - Header

    pub fn decode_null_header(&mut self) -> Result<NullHeader> {
        let _ = self.pull_byte_expecting(Marker::Null)?;

        Ok(NullHeader)
    }

    // MARK: - Body

    pub fn decode_null_value_of(&mut self, header: NullHeader) -> Result<NullValue> {
        let _ = header;

        Ok(NullValue)
    }
}
