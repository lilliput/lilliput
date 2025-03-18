use crate::{
    error::Result,
    header::{EncodeHeader as _, SeqHeader},
    io::Write,
    value::{SeqValue, Value},
    Profile,
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
        // Push the value's header:
        let header = match self.profile {
            Profile::Weak => SeqHeader::optimal(len),
            Profile::None => SeqHeader::extended(8),
        };
        self.push_bytes(&[header.encode()])?;

        // Push the value's length extension:
        if let SeqHeader::Extended { len_width } = header {
            let len_bytes = len.to_be_bytes();
            let len_bytes_start = len_bytes.len() - len_width;
            self.push_bytes(&len_bytes[len_bytes_start..])?;
        }

        Ok(())
    }
}
