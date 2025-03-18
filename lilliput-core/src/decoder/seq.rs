use crate::{
    error::Result,
    header::SeqHeader,
    io::Read,
    value::{SeqValue, Value},
};

use super::Decoder;

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    pub fn decode_seq(&mut self) -> Result<Vec<Value>> {
        let header: SeqHeader = self.pull_header()?;
        self.decode_seq_headed_by(header)
    }

    pub fn decode_seq_start(&mut self) -> Result<usize> {
        let header: SeqHeader = self.pull_header()?;
        self.decode_seq_start_headed_by(header)
    }

    pub fn decode_seq_value(&mut self) -> Result<SeqValue> {
        let header: SeqHeader = self.pull_header()?;
        self.decode_seq_value_headed_by(header)
    }

    fn decode_seq_headed_by(&mut self, header: SeqHeader) -> Result<Vec<Value>> {
        let len = self.decode_seq_start_headed_by(header)?;
        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            let value = self.decode_any()?;
            vec.push(value);
        }

        self.decode_seq_end()?;

        Ok(vec)
    }

    pub(super) fn decode_seq_value_headed_by(&mut self, header: SeqHeader) -> Result<SeqValue> {
        self.decode_seq_headed_by(header).map(From::from)
    }

    fn decode_seq_start_headed_by(&mut self, header: SeqHeader) -> Result<usize> {
        let len = match header {
            SeqHeader::Compact { len } => len,
            SeqHeader::Extended { len_width } => self.pull_len_bytes(len_width)?,
        };

        Ok(len)
    }

    pub fn decode_seq_end(&mut self) -> Result<()> {
        Ok(())
    }
}
