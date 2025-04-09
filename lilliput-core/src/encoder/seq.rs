use crate::{
    error::Result,
    header::{CompactSeqHeader, ExtendedSeqHeader, SeqHeader},
    io::Write,
    num::WithPackedBeBytes as _,
    value::{SeqValue, Value},
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_seq(&mut self, value: &[Value]) -> Result<()> {
        self.encode_seq_header(&self.header_for_seq(value))?;

        for value in value {
            self.encode_value(value)?;
        }

        Ok(())
    }

    pub fn encode_seq_value(&mut self, value: &SeqValue) -> Result<()> {
        self.encode_seq(&value.0)
    }

    pub fn encode_seq_header(&mut self, header: &SeqHeader) -> Result<()> {
        let mut header_byte = SeqHeader::TYPE_BITS;

        match *header {
            SeqHeader::Compact(CompactSeqHeader { len }) => {
                header_byte |= SeqHeader::COMPACT_VARIANT_BIT;
                header_byte |= len & SeqHeader::COMPACT_LEN_BITS;

                // Push the value's header:
                self.push_byte(header_byte)
            }
            SeqHeader::Extended(ExtendedSeqHeader { len }) => {
                len.with_packed_be_bytes(self.config.len_packing, |bytes| {
                    let width = bytes.len() as u8;

                    header_byte |= (width - 1) & SeqHeader::EXTENDED_LEN_WIDTH_BITS;

                    // Push the value's header:
                    self.push_byte(header_byte)?;

                    // Push the value's length:
                    self.push_bytes(bytes)
                })
            }
        }
    }

    pub fn header_for_seq(&self, seq: &[Value]) -> SeqHeader {
        SeqHeader::for_len(seq.len(), self.config.len_packing)
    }
}
