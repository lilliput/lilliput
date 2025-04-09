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
    // MARK: - Value

    pub fn encode_str(&mut self, value: &str) -> Result<()> {
        self.encode_string_header(&self.header_for_str(value))?;

        // Push the value's actual bytes:
        self.push_bytes(value.as_bytes())?;

        Ok(())
    }

    pub fn encode_string_value(&mut self, value: &StringValue) -> Result<()> {
        self.encode_str(&value.0)?;

        Ok(())
    }

    // MARK: - Header

    pub fn encode_string_header(&mut self, header: &StringHeader) -> Result<()> {
        let mut byte = StringHeader::TYPE_BITS;

        match *header {
            StringHeader::Compact(CompactStringHeader { len }) => {
                byte |= StringHeader::COMPACT_VARIANT_BIT;
                byte |= len & StringHeader::COMPACT_LEN_BITS;

                // Push the value's header:
                self.push_byte(byte)
            }
            StringHeader::Extended(ExtendedStringHeader { len }) => {
                len.with_packed_be_bytes(self.config.len_packing, |bytes| {
                    let width = bytes.len() as u8;

                    byte |= (width - 1) & StringHeader::EXTENDED_LEN_WIDTH_BITS;

                    // Push the value's header:
                    self.push_byte(byte)?;

                    // Push the value's length:
                    self.push_bytes(bytes)
                })
            }
        }
    }

    pub fn header_for_str(&self, str: &str) -> StringHeader {
        StringHeader::for_len(str.len(), self.config.len_packing)
    }
}
