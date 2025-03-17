use crate::{
    header::SeqHeader,
    value::{SeqValue, Value},
};

use super::{BufRead, Decoder, DecoderError};

#[derive(Debug)]
pub struct SeqDecoder<'de, R> {
    inner: &'de mut Decoder<R>,
}

impl<'de, R> SeqDecoder<'de, R>
where
    R: BufRead,
{
    pub(super) fn with(inner: &'de mut Decoder<R>) -> Self {
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
        let header: SeqHeader = self.inner.pull_header()?;

        let len = match header {
            SeqHeader::Compact { len } => len,
            SeqHeader::Extended { len_width } => self.inner.pull_len_bytes(len_width)?,
        };

        Ok(len)
    }

    pub(super) fn decode_seq_end(&mut self) -> Result<(), DecoderError> {
        Ok(())
    }
}
