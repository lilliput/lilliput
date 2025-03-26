use crate::{
    error::Result,
    header::{SeqHeader, SeqHeaderRepr},
    io::Read,
    value::{SeqValue, Value},
};

use super::Decoder;

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    pub fn decode_seq(&mut self) -> Result<Vec<Value>> {
        let len = self.decode_seq_start()?;
        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            let value = self.decode_any()?;
            vec.push(value);
        }

        Ok(vec)
    }

    pub fn decode_seq_start(&mut self) -> Result<usize> {
        let header: SeqHeader = self.pull_header()?;

        let len: usize = match header.repr() {
            SeqHeaderRepr::Compact { len } => len.into(),
            SeqHeaderRepr::Extended { len_width } => self.pull_len_bytes(len_width)?,
        };

        Ok(len)
    }

    pub fn decode_seq_value(&mut self) -> Result<SeqValue> {
        self.decode_seq().map(From::from)
    }
}
