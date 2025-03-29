use crate::{
    error::Result,
    header::{EncodeHeader as _, StringHeader},
    io::Write,
    num::int::with_n_be_bytes,
    value::StringValue,
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_str(&mut self, value: &str) -> Result<()> {
        let len = value.len();

        // Push the value's header and length:
        self.encode_str_header(len)?;

        // Push the value's actual bytes:
        self.push_bytes(value.as_bytes())?;

        Ok(())
    }

    pub fn encode_string_value(&mut self, value: &StringValue) -> Result<()> {
        self.encode_str(&value.0)?;

        Ok(())
    }

    pub fn encode_str_header(&mut self, len: usize) -> Result<()> {
        let header = if self.config.compact_ints {
            StringHeader::optimal(len)
        } else {
            StringHeader::verbatim(len)
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
