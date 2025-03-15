use crate::value::{BytesValue, ValueType};

use super::{Decoder, DecoderError};

#[derive(Debug)]
pub struct BytesDecoder<'a, 'de> {
    inner: &'de mut Decoder<'a>,
}

impl<'a, 'de> BytesDecoder<'a, 'de> {
    pub(super) fn with(inner: &'de mut Decoder<'a>) -> Self {
        Self { inner }
    }

    pub(super) fn decode_bytes(&mut self) -> Result<Vec<u8>, DecoderError> {
        let byte = self.inner.pull_byte_expecting_type(ValueType::Bytes)?;

        let len_width_exponent = (byte & BytesValue::LONG_WIDTH_BITS) as u32;
        let len_width = 1_usize << len_width_exponent;

        let len = {
            let range = {
                let start = 8 - len_width;
                let end = start + len_width;
                start..end
            };

            let mut len_bytes: [u8; 8] = [0b0; 8];
            let bytes = self.inner.pull_bytes(len_width)?;
            len_bytes[range].copy_from_slice(bytes);

            u64::from_be_bytes(len_bytes) as usize
        };

        let remaining_len = self.inner.remaining_len();
        debug_assert!(len <= remaining_len);

        let capacity = len.min(remaining_len);
        let mut value = Vec::with_capacity(capacity);
        value.extend_from_slice(self.inner.pull_bytes(len)?);

        Ok(value)
    }

    pub(super) fn decode_bytes_value(&mut self) -> Result<BytesValue, DecoderError> {
        self.decode_bytes().map(From::from)
    }
}
