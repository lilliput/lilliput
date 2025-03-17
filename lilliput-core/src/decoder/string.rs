use crate::{header::StringHeader, value::StringValue};

use super::{BufRead, Decoder, DecoderError};

#[derive(Debug)]
pub struct StringDecoder<'de, R> {
    inner: &'de mut Decoder<R>,
}

impl<'de, R> StringDecoder<'de, R>
where
    R: BufRead,
{
    pub(super) fn with(inner: &'de mut Decoder<R>) -> Self {
        Self { inner }
    }

    pub(super) fn decode_string(&mut self) -> Result<String, DecoderError> {
        let header: StringHeader = self.inner.pull_header()?;

        let len = match header {
            StringHeader::Compact { len } => len,
            StringHeader::Extended { len_width } => self.inner.pull_len_bytes(len_width)?,
        };

        // We cannot trust the decoded length, so we only ever
        // allocate as much bytes as we know (with certainty)
        // to be remaining in the incoming byte stream:

        let capacity = len.min(self.inner.peek_bytes()?.len());
        let mut buf = Vec::with_capacity(capacity);

        let mut pos: usize = 0;

        while pos < len {
            let peek_buf = self.inner.peek_bytes()?;
            let pull_len = (len - pos).min(peek_buf.len());
            buf.extend_from_slice(&peek_buf[0..pull_len]);
            self.inner.consume_bytes(pull_len)?;

            pos += pull_len;
        }

        let value = String::from_utf8(buf)?;

        Ok(value)
    }

    pub(super) fn decode_string_value(&mut self) -> Result<StringValue, DecoderError> {
        self.decode_string().map(From::from)
    }
}
