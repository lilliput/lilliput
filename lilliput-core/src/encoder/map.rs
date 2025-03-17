use crate::{
    header::{EncodeHeader as _, MapHeader},
    io::Write,
    value::{Map, MapValue},
    Profile,
};

use super::{Encoder, EncoderError};

#[derive(Debug)]
pub(super) struct MapEncoder<'en, W> {
    inner: &'en mut Encoder<W>,
}

impl<'en, W> MapEncoder<'en, W>
where
    W: Write,
{
    pub(super) fn with(inner: &'en mut Encoder<W>) -> Self {
        Self { inner }
    }

    pub(super) fn encode_map(&mut self, value: &Map) -> Result<(), EncoderError> {
        self.encode_map_start(value.len())?;

        for (key, value) in value {
            self.inner.encode_any(key)?;
            self.inner.encode_any(value)?;
        }

        self.encode_map_end()
    }

    pub(super) fn encode_map_value(&mut self, value: &MapValue) -> Result<(), EncoderError> {
        self.encode_map(&value.0)
    }

    pub(super) fn encode_map_start(&mut self, len: usize) -> Result<(), EncoderError> {
        // Push the value's header:
        let header = match self.inner.profile {
            Profile::Weak => MapHeader::optimal(len),
            Profile::None => MapHeader::extended(8),
        };
        self.inner.push_bytes(&[header.encode()])?;

        // Push the value's length extension:
        if let MapHeader::Extended { len_width } = header {
            let len_bytes = len.to_be_bytes();
            let len_bytes_start = len_bytes.len() - len_width;
            self.inner.push_bytes(&len_bytes[len_bytes_start..])?;
        }

        Ok(())
    }

    pub(super) fn encode_map_end(&mut self) -> Result<(), EncoderError> {
        Ok(())
    }
}
