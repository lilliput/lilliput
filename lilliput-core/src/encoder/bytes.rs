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

        // Push the value's header:
        let header = BytesHeader::optimal(len);
        self.push_bytes(&[header.encode()])?;

        // Push the value's length:
        match header.len_width() {
            8 => self.push_bytes(&len.to_be_bytes())?,
            len_width => {
                debug_assert!(len_width <= 8);

                const MAX_LEN_WIDTH: usize = 8;
                let len_bytes: [u8; 8] = len.to_be_bytes();
                let len_bytes_start = MAX_LEN_WIDTH - len_width;
                self.push_bytes(&len_bytes[len_bytes_start..])?;
            }
        }

        // Push the value's actual bytes:
        let tail_bytes = value;
        self.push_bytes(tail_bytes)?;

        Ok(())
    }

    pub fn encode_bytes_value(&mut self, value: &BytesValue) -> Result<()> {
        self.encode_bytes(&value.0)
    }
}
