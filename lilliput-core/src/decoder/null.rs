use crate::{error::Result, header::NullHeader, value::NullValue};

use super::{Decoder, Read};

impl<'r, R> Decoder<R>
where
    R: Read<'r>,
{
    pub fn decode_null(&mut self) -> Result<()> {
        let header: NullHeader = self.pull_header()?;
        self.decode_null_headed_by(header)
    }

    pub fn decode_null_value(&mut self) -> Result<NullValue> {
        let header: NullHeader = self.pull_header()?;
        self.decode_null_value_headed_by(header)
    }

    fn decode_null_headed_by(&mut self, header: NullHeader) -> Result<()> {
        let _ = header;

        Ok(())
    }

    pub(super) fn decode_null_value_headed_by(&mut self, header: NullHeader) -> Result<NullValue> {
        self.decode_null_headed_by(header)?;

        Ok(NullValue::default())
    }
}
