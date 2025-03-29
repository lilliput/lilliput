use crate::{
    error::Result,
    header::{EncodeHeader as _, SeqHeader},
    io::Write,
    num::int::with_n_be_bytes,
    value::{SeqValue, Value},
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_seq(&mut self, value: &[Value]) -> Result<()> {
        self.encode_seq_start(value.len())?;

        for value in value {
            self.encode_any(value)?;
        }

        Ok(())
    }

    pub fn encode_seq_value(&mut self, value: &SeqValue) -> Result<()> {
        self.encode_seq(&value.0)
    }

    pub fn encode_seq_start(&mut self, len: usize) -> Result<()> {
        let header = if self.config.compact_ints {
            SeqHeader::optimal(len)
        } else {
            SeqHeader::verbatim(len)
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
