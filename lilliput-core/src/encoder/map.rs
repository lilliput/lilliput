use crate::value::{Map, MapValue};

use super::{Encoder, EncoderError};

#[derive(Debug)]
pub(super) struct MapEncoder<'en> {
    inner: &'en mut Encoder,
}

impl<'en> MapEncoder<'en> {
    pub(super) fn with(inner: &'en mut Encoder) -> Self {
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
        // Push the value's metadata:
        let mut head_byte = MapValue::PREFIX_BIT;
        head_byte |= MapValue::VARIANT_BIT;
        head_byte |= 3; // width exponent of usize (2 ^ 3 = 8)
        self.inner.push_byte(head_byte)?;

        // Push the value's length:
        let neck_bytes = len.to_be_bytes();
        self.inner.push_bytes(&neck_bytes)?;

        Ok(())
    }

    pub(super) fn encode_map_end(&mut self) -> Result<(), EncoderError> {
        Ok(())
    }
}
