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
    // MARK: - Value

    /// Encodes a sequence value.
    pub fn encode_seq(&mut self, value: &[Value]) -> Result<()> {
        self.encode_seq_header(&self.header_for_seq_len(value.len()))?;

        for value in value {
            self.encode_value(value)?;
        }

        Ok(())
    }

    /// Encodes a sequence value, from a `SeqValue`.
    pub fn encode_seq_value(&mut self, value: &SeqValue) -> Result<()> {
        self.encode_seq(&value.0)
    }

    // MARK: - Header

    /// Encodes a sequence value's header.
    pub fn encode_seq_header(&mut self, header: &SeqHeader) -> Result<()> {
        let mut byte = SeqHeader::TYPE_BITS;

        match *header {
            SeqHeader::Compact(CompactSeqHeader { len }) => {
                byte |= SeqHeader::COMPACT_VARIANT_BIT;
                byte |= len & SeqHeader::COMPACT_LEN_BITS;

                // Push the value's header:
                self.push_byte(byte)
            }
            SeqHeader::Extended(ExtendedSeqHeader { len }) => {
                len.with_packed_be_bytes(self.config.lengths.packing, |bytes| {
                    let width = bytes.len() as u8;

                    byte |= (width - 1) & SeqHeader::EXTENDED_LEN_WIDTH_BITS;

                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        byte = crate::binary::fmt_byte(byte),
                        bytes = format!("{:b}", crate::binary::BytesSlice(bytes)),
                        len = len
                    );

                    // Push the value's header:
                    self.push_byte(byte)?;

                    // Push the value's length:
                    self.push_bytes(bytes)
                })
            }
        }
    }

    /// Creates a header for a sequence value, from its length.
    pub fn header_for_seq_len(&self, len: usize) -> SeqHeader {
        SeqHeader::for_len(len, self.config.lengths.packing)
    }
}
