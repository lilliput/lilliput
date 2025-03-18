use crate::{
    error::Result,
    header::{EncodeHeader as _, StringHeader},
    io::Write,
    value::StringValue,
    Profile,
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_str(&mut self, value: &str) -> Result<()> {
        let len = value.len();

        // Push the value's header:
        let header = match self.profile {
            Profile::Weak => StringHeader::optimal(len),
            Profile::None => StringHeader::extended(8),
        };
        self.push_bytes(&[header.encode()])?;

        // Push the value's length extension:
        if let StringHeader::Extended { len_width } = header {
            let len_bytes = len.to_be_bytes();
            let len_bytes_start = len_bytes.len() - len_width;
            self.push_bytes(&len_bytes[len_bytes_start..])?;
        }

        // Push the value's actual bytes:
        let tail_bytes = value.as_bytes();
        self.push_bytes(tail_bytes)?;

        Ok(())
    }

    pub fn encode_string_value(&mut self, value: &StringValue) -> Result<()> {
        self.encode_str(&value.0)?;

        Ok(())
    }
}
