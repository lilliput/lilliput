use crate::{
    header::{EncodeHeader as _, SeqHeader},
    io::Write,
    value::{SeqValue, Value},
    Profile,
};

use super::{Encoder, EncoderError};

#[derive(Debug)]
pub(super) struct SeqEncoder<'en, W> {
    inner: &'en mut Encoder<W>,
}

impl<'en, W> SeqEncoder<'en, W>
where
    W: Write,
{
    pub(super) fn with(inner: &'en mut Encoder<W>) -> Self {
        Self { inner }
    }

    pub(super) fn encode_seq(&mut self, value: &[Value]) -> Result<(), EncoderError> {
        self.encode_seq_start(value.len())?;

        for value in value {
            self.inner.encode_any(value)?;
        }

        self.encode_seq_end()
    }

    pub(super) fn encode_seq_value(&mut self, value: &SeqValue) -> Result<(), EncoderError> {
        self.encode_seq(&value.0)
    }

    pub(super) fn encode_seq_start(&mut self, len: usize) -> Result<(), EncoderError> {
        // Push the value's header:
        let header = match self.inner.profile {
            Profile::Weak => SeqHeader::optimal(len),
            Profile::None => SeqHeader::extended(8),
        };
        self.inner.push_bytes(&[header.encode()])?;

        // Push the value's length extension:
        if let SeqHeader::Extended { len_width } = header {
            let len_bytes = len.to_be_bytes();
            let len_bytes_start = len_bytes.len() - len_width;
            self.inner.push_bytes(&len_bytes[len_bytes_start..])?;
        }

        Ok(())
    }

    pub(super) fn encode_seq_end(&mut self) -> Result<(), EncoderError> {
        Ok(())
    }
}
