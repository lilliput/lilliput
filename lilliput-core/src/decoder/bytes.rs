use crate::{header::BytesHeader, value::BytesValue};

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
        let header: BytesHeader = self.inner.pull_header()?;

        let len_width = header.len_width();
        let len = self.inner.pull_len_bytes(len_width)?;

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
