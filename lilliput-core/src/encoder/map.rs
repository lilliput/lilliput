use crate::{
    error::Result,
    header::{EncodeHeader as _, MapHeader},
    io::Write,
    num::int::with_n_be_bytes,
    value::{Map, MapValue},
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_map(&mut self, value: &Map) -> Result<()> {
        self.encode_map_header(value.len())?;

        for (key, value) in value {
            self.encode_any(key)?;
            self.encode_any(value)?;
        }

        Ok(())
    }

    pub fn encode_map_value(&mut self, value: &MapValue) -> Result<()> {
        self.encode_map(&value.0)
    }

    pub fn encode_map_header(&mut self, len: usize) -> Result<()> {
        let header = if self.config.compact_ints {
            MapHeader::optimal(len)
        } else {
            MapHeader::verbatim(len)
        };

        // Push the value's header:
        self.push_bytes(&[header.encode()])?;

        if let Some(len_width) = header.extension_width() {
            with_n_be_bytes(len, len_width, |len_bytes| {
                // Push the value's length extension:
                self.push_bytes(len_bytes)
            })?;
        }

        Ok(())
    }
}
