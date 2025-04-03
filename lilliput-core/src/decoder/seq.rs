use crate::{
    error::Result,
    header::SeqHeader,
    io::Read,
    marker::Marker,
    value::{SeqValue, Value},
};

use super::Decoder;

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    pub fn decode_seq(&mut self) -> Result<Vec<Value>> {
        let header = self.decode_seq_header()?;
        let mut vec = Vec::new();

        for _ in 0..header.len() {
            let value = self.decode_any()?;
            vec.push(value);
        }

        Ok(vec)
    }

    pub fn decode_seq_value(&mut self) -> Result<SeqValue> {
        self.decode_seq().map(From::from)
    }

    pub fn decode_seq_header(&mut self) -> Result<SeqHeader> {
        let header_byte = self.pull_byte_expecting(Marker::Seq)?;

        let is_compact = (header_byte & SeqHeader::COMPACT_VARIANT_BIT) != 0b0;

        if is_compact {
            let len = header_byte & SeqHeader::COMPACT_LEN_BITS;
            Ok(SeqHeader::compact(len))
        } else {
            let len_width = 1 + (header_byte & SeqHeader::EXTENDED_LEN_WIDTH_BITS);
            let len = self.pull_len_bytes(len_width)?;
            Ok(SeqHeader::extended(len))
        }
    }
}
