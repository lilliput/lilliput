use crate::{
    header::{EncodeHeader as _, StringHeader},
    value::StringValue,
    Profile,
};

use super::{Encoder, EncoderError};

#[derive(Debug)]
pub(super) struct StringEncoder<'en> {
    inner: &'en mut Encoder,
}

impl<'en> StringEncoder<'en> {
    pub(super) fn with(inner: &'en mut Encoder) -> Self {
        Self { inner }
    }

    pub(super) fn encode_string(&mut self, value: &str) -> Result<(), EncoderError> {
        let len = value.len();

        // Push the value's header:
        let header = match self.inner.profile {
            Profile::Weak => StringHeader::optimal(len),
            Profile::None => StringHeader::extended(8),
        };
        println!("header for len {len}: {header:?}");
        self.inner.push_byte(header.encode())?;
        println!("header encoded: {:08b}", header.encode());

        // Push the value's length extension:
        if let StringHeader::Extended { len_width } = header {
            let len_bytes = len.to_be_bytes();
            let len_bytes_start = len_bytes.len() - len_width;
            self.inner.push_bytes(&len_bytes[len_bytes_start..])?;
        }

        // Push the value's actual bytes:
        let tail_bytes = value.as_bytes();
        self.inner.push_bytes(tail_bytes)?;

        Ok(())
    }

    pub(super) fn encode_string_value(&mut self, value: &StringValue) -> Result<(), EncoderError> {
        self.encode_string(&value.0)?;

        Ok(())
    }
}
