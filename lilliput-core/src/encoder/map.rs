use crate::{
    config::PackingMode,
    error::Result,
    header::{CompactMapHeader, ExtendedMapHeader, MapHeader},
    io::Write,
    num::WithPackedBeBytes as _,
    value::{Map, MapValue},
};

use super::Encoder;

impl<W> Encoder<W>
where
    W: Write,
{
    pub fn encode_map(&mut self, value: &Map) -> Result<()> {
        self.encode_map_header(&self.header_for_map(value.len()))?;

        for (key, value) in value {
            self.encode_value(key)?;
            self.encode_value(value)?;
        }

        Ok(())
    }

    pub fn encode_map_value(&mut self, value: &MapValue) -> Result<()> {
        self.encode_map(&value.0)
    }

    pub fn encode_map_header(&mut self, header: &MapHeader) -> Result<()> {
        let mut header_byte = MapHeader::TYPE_BITS;

        match *header {
            MapHeader::Compact(CompactMapHeader { len }) => {
                header_byte |= MapHeader::COMPACT_VARIANT_BIT;
                header_byte |= len & MapHeader::COMPACT_LEN_BITS;

                // Push the value's header:
                self.push_byte(header_byte)
            }
            MapHeader::Extended(ExtendedMapHeader { len }) => {
                len.with_packed_be_bytes(self.config.len_packing, |bytes| {
                    let width = bytes.len() as u8;

                    header_byte |= (width - 1) & MapHeader::EXTENDED_LEN_WIDTH_BITS;

                    // Push the value's header:
                    self.push_byte(header_byte)?;

                    // Push the value's length:
                    self.push_bytes(bytes)
                })
            }
        }
    }

    pub fn header_for_map(&self, len: usize) -> MapHeader {
        let allows_compact = self.config.len_packing == PackingMode::Optimal;

        if allows_compact && len <= (MapHeader::COMPACT_LEN_BITS as usize) {
            MapHeader::compact_unchecked(len as u8)
        } else {
            MapHeader::extended(len)
        }
    }
}
