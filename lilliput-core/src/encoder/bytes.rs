use crate::{
    header::{BytesHeader, EncodeHeader},
    value::BytesValue,
};

use super::{Encoder, EncoderError};

#[derive(Debug)]
pub(super) struct BytesEncoder<'en> {
    inner: &'en mut Encoder,
}

impl<'en> BytesEncoder<'en> {
    pub(super) fn with(inner: &'en mut Encoder) -> Self {
        Self { inner }
    }

    pub(super) fn encode_bytes(&mut self, value: &[u8]) -> Result<(), EncoderError> {
        let len = value.len();

        // Push the value's header:
        let header = BytesHeader::optimal(len);
        self.inner.push_byte(header.encode())?;

        // Push the value's length:
        match header.len_width() {
            8 => self.inner.push_bytes(&len.to_be_bytes())?,
            len_width => {
                debug_assert!(len_width <= 8);

                const MAX_LEN_WIDTH: usize = 8;
                let len_bytes: [u8; 8] = len.to_be_bytes();
                let len_bytes_start = MAX_LEN_WIDTH - len_width;
                self.inner.push_bytes(&len_bytes[len_bytes_start..])?;
            }
        }

        // Push the value's actual bytes:
        let tail_bytes = value;
        self.inner.push_bytes(tail_bytes)?;

        Ok(())
    }

    pub(super) fn encode_bytes_value(&mut self, value: &BytesValue) -> Result<(), EncoderError> {
        self.encode_bytes(&value.0)
    }
}
