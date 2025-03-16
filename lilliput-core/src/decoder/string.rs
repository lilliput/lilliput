use crate::{header::StringHeader, value::StringValue};

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
        let header: StringHeader = self.inner.pull_header()?;
        println!("header decoded: {header:?}");

        let len = match header {
            StringHeader::Compact { len } => len,
            StringHeader::Extended { len_width } => self.inner.pull_len_bytes(len_width)?,
        };

        if self.inner.remaining_len() < len {
            return Err(DecoderError::Eof);
        }

        let bytes = self.inner.pull_bytes(len)?;
        let value = String::from_utf8(bytes.to_owned())?;

        Ok(value)
    }

    pub(super) fn decode_string_value(&mut self) -> Result<StringValue, DecoderError> {
        self.decode_string().map(From::from)
    }
}
