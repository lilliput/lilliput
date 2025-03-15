use crate::value::{SeqValue, Value, ValueType};

use super::{Decoder, DecoderError};

#[derive(Debug)]
pub struct SeqDecoder<'a, 'de> {
    inner: &'de mut Decoder<'a>,
}

impl<'a, 'de> SeqDecoder<'a, 'de> {
    pub(super) fn with(inner: &'de mut Decoder<'a>) -> Self {
        Self { inner }
    }

    pub(super) fn decode_seq(&mut self) -> Result<Vec<Value>, DecoderError> {
        let len = self.decode_seq_start()?;
        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            let value = self.inner.decode_any()?;
            vec.push(value);
        }

        self.decode_seq_end()?;

        Ok(vec)
    }

    pub(super) fn decode_seq_value(&mut self) -> Result<SeqValue, DecoderError> {
        self.decode_seq().map(From::from)
    }

    pub(super) fn decode_seq_start(&mut self) -> Result<usize, DecoderError> {
        let byte = self.inner.pull_byte_expecting_type(ValueType::Seq)?;

        let is_long = byte & SeqValue::VARIANT_BIT != 0b0;

        if is_long {
            let is_valid = byte & SeqValue::LONG_RESERVED_BIT == 0b0;
            let len_width = (byte & SeqValue::LONG_LEN_WIDTH_BITS) as usize + 1;

            assert!(is_valid, "padding bits should be zero");

            let mut bytes: [u8; 8] = [0b0; 8];

            let range = {
                let start = 8 - len_width;
                let end = start + len_width;
                start..end
            };

            bytes[range].copy_from_slice(self.inner.pull_bytes(len_width)?);

            let len = u64::from_be_bytes(bytes) as usize;

            if self.inner.remaining_len() < len {
                return Err(DecoderError::Eof);
            }

            Ok(len)
        } else {
            Err(DecoderError::IncompatibleProfile)
        }
    }

    pub(super) fn decode_seq_end(&mut self) -> Result<(), DecoderError> {
        Ok(())
    }
}
