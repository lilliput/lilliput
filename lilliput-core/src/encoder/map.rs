use crate::{
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
    // MARK: - Value

    pub fn encode_map(&mut self, value: &Map) -> Result<()> {
        self.encode_map_header(&self.header_for_map(value))?;

        for (key, value) in value {
            self.encode_value(key)?;
            self.encode_value(value)?;
        }

        Ok(())
    }

    pub fn encode_map_value(&mut self, value: &MapValue) -> Result<()> {
        self.encode_map(&value.0)
    }

    // MARK: - Header

    pub fn encode_map_header(&mut self, header: &MapHeader) -> Result<()> {
        let mut byte = MapHeader::TYPE_BITS;

        match *header {
            MapHeader::Compact(CompactMapHeader { len }) => {
                byte |= MapHeader::COMPACT_VARIANT_BIT;
                byte |= len & MapHeader::COMPACT_LEN_BITS;

                // Push the value's header:
                self.push_byte(byte)
            }
            MapHeader::Extended(ExtendedMapHeader { len }) => {
                len.with_packed_be_bytes(self.config.len_packing, |bytes| {
                    let width = bytes.len() as u8;

                    byte |= (width - 1) & MapHeader::EXTENDED_LEN_WIDTH_BITS;

                    // Push the value's header:
                    self.push_byte(byte)?;

                    // Push the value's length:
                    self.push_bytes(bytes)
                })
            }
        }
    }

    pub fn header_for_map(&self, map: &Map) -> MapHeader {
        MapHeader::for_len(map.len(), self.config.len_packing)
    }
}
