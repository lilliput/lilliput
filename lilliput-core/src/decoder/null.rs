use crate::{error::Result, header::NullHeader, value::NullValue};

use super::{Decoder, Read};

impl<'r, R> Decoder<R>
where
    R: Read<'r>,
{
    pub fn decode_null(&mut self) -> Result<()> {
        let _: NullHeader = self.pull_header()?;
        Ok(())
    }

    pub fn decode_null_value(&mut self) -> Result<NullValue> {
        self.decode_null().map(From::from)
    }
}
