use crate::value::{StringValue, ValueType};

use super::{Decoder, DecoderError};

#[derive(Debug)]
pub struct StringDecoder<'a, 'de> {
    inner: &'de mut Decoder<'a>,
}

impl<'a, 'de> StringDecoder<'a, 'de> {
    pub(super) fn with(inner: &'de mut Decoder<'a>) -> Self {
        Self { inner }
    }

    pub(super) fn decode_string(&mut self) -> Result<String, DecoderError> {
        let byte = self.inner.pull_byte_expecting_type(ValueType::String)?;

        if byte & StringValue::COMPACTNESS_BIT != 0b0 {
            // Support for compact coding is not implemented yet.
            return Err(DecoderError::IncompatibleProfile);
        }

        let is_valid = byte & StringValue::LONG_RESERVED_BITS == 0b0;
        let len_width = (byte & StringValue::LONG_LEN_WIDTH_BITS) as usize + 1;

        assert!(is_valid, "padding bits should be zero");

        let mut bytes: [u8; 8] = [0b0; 8];

        let range = {
            let start = 8 - len_width;
            let end = start + len_width;
            start..end
        };

        bytes[range].copy_from_slice(self.inner.pull_bytes(len_width)?);

        let len = u64::from_be_bytes(bytes) as usize;

        let mut bytes = Vec::with_capacity(len.min(self.inner.remaining_len()));
        bytes.extend_from_slice(self.inner.pull_bytes(len)?);

        let value = String::from_utf8(bytes)?;

        Ok(value)
    }

    pub(super) fn decode_string_value(&mut self) -> Result<StringValue, DecoderError> {
        self.decode_string().map(From::from)
    }
}
