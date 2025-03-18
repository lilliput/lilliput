use crate::{
    error::Result,
    header::{EncodeHeader as _, MapHeader},
    io::Write,
    value::{Map, MapValue},
    Profile,
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_map(&mut self, value: &Map) -> Result<()> {
        self.encode_map_start(value.len())?;

        for (key, value) in value {
            self.encode_any(key)?;
            self.encode_any(value)?;
        }

        Ok(())
    }

    pub fn encode_map_value(&mut self, value: &MapValue) -> Result<()> {
        self.encode_map(&value.0)
    }

    pub fn encode_map_start(&mut self, len: usize) -> Result<()> {
        // Push the value's header:
        let header = match self.profile {
            Profile::Weak => MapHeader::optimal(len),
            Profile::None => MapHeader::extended(8),
        };
        self.push_bytes(&[header.encode()])?;

        // Push the value's length extension:
        if let MapHeader::Extended { len_width } = header {
            let len_bytes = len.to_be_bytes();
            let len_bytes_start = len_bytes.len() - len_width;
            self.push_bytes(&len_bytes[len_bytes_start..])?;
        }

        Ok(())
    }
}
