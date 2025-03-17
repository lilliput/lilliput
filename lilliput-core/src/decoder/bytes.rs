use crate::{header::BytesHeader, value::BytesValue};

use super::{BufRead, Decoder, DecoderError};

#[derive(Debug)]
pub struct BytesDecoder<'de, R> {
    inner: &'de mut Decoder<R>,
}

impl<'de, R> BytesDecoder<'de, R>
where
    R: BufRead,
{
    pub(super) fn with(inner: &'de mut Decoder<R>) -> Self {
        Self { inner }
    }

    pub(super) fn decode_bytes(&mut self) -> Result<Vec<u8>, DecoderError> {
        let header: BytesHeader = self.inner.pull_header()?;

        let len_width = header.len_width();
        let len = self.inner.pull_len_bytes(len_width)?;

        // We cannot trust the decoded length, so we only ever
        // allocate as much bytes as we know (with certainty)
        // to be remaining in the incoming byte stream:

        let capacity = len.min(self.inner.peek_bytes()?.len());
        let mut value = Vec::with_capacity(capacity);

        let mut pos: usize = 0;

        while pos < len {
            let peek_buf = self.inner.peek_bytes()?;
            let pull_len = (len - pos).min(peek_buf.len());
            value.extend_from_slice(&peek_buf[0..pull_len]);
            self.inner.consume_bytes(pull_len)?;

            pos += pull_len;
        }

        Ok(value)
    }

    pub(super) fn decode_bytes_value(&mut self) -> Result<BytesValue, DecoderError> {
        self.decode_bytes().map(From::from)
    }
}
