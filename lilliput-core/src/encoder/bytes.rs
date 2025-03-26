use crate::{
    error::Result,
    header::{BytesHeader, EncodeHeader},
    io::Write,
    value::BytesValue,
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_bytes(&mut self, value: &[u8]) -> Result<()> {
        let len = value.len();

        // Push the value's header and length:
        self.encode_bytes_start(len)?;

        // Push the value's actual bytes:
        let tail_bytes = value;
        self.push_bytes(tail_bytes)?;

        Ok(())
    }

    pub fn encode_bytes_value(&mut self, value: &BytesValue) -> Result<()> {
        self.encode_bytes(&value.0)
    }

    pub fn encode_bytes_start(&mut self, len: usize) -> Result<()> {
        let header = BytesHeader::optimal(len);

        // Push the value's header:
        self.push_bytes(&[header.encode()])?;

        // Push the value's length:
        const MAX_LEN_WIDTH: usize = 8;
        let len_bytes: [u8; 8] = len.to_be_bytes();
        let len_width: usize = header.len_width().into();
        let len_bytes_start = MAX_LEN_WIDTH - len_width;
        self.push_bytes(&len_bytes[len_bytes_start..])?;

        Ok(())
    }
}
