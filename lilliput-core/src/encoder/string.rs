use crate::{
    error::Result,
    header::{CompactStringHeader, ExtendedStringHeader, StringHeader},
    io::Write,
    num::WithPackedBeBytes as _,
    value::StringValue,
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_str(&mut self, value: &str) -> Result<()> {
        let packing_mode = self.config.len_packing;

        self.encode_string_header(&StringHeader::new(value.len(), packing_mode))?;

        // Push the value's actual bytes:
        self.push_bytes(value.as_bytes())?;

        Ok(())
    }

    pub fn encode_string_value(&mut self, value: &StringValue) -> Result<()> {
        self.encode_str(&value.0)?;

        Ok(())
    }

    pub fn encode_string_header(&mut self, header: &StringHeader) -> Result<()> {
        let mut header_byte = StringHeader::TYPE_BITS;

        match *header {
            StringHeader::Compact(CompactStringHeader { len }) => {
                header_byte |= StringHeader::COMPACT_VARIANT_BIT;
                header_byte |= len & StringHeader::COMPACT_LEN_BITS;

                // Push the value's header:
                self.push_byte(header_byte)
            }
            StringHeader::Extended(ExtendedStringHeader { len }) => {
                len.with_packed_be_bytes(self.config.len_packing, |width, bytes| {
                    header_byte |= (width - 1) & StringHeader::EXTENDED_LEN_WIDTH_BITS;

                    // Push the value's header:
                    self.push_byte(header_byte)?;

                    // Push the value's length:
                    self.push_bytes(bytes)
                })
            }
        }
    }
}
