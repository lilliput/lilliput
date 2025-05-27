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

    /// Encodes a map value.
    pub fn encode_map(&mut self, value: &Map) -> Result<()> {
        self.encode_map_header(&self.header_for_map_len(value.len()))?;

        for (key, value) in value {
            self.encode_value(key)?;
            self.encode_value(value)?;
        }

        Ok(())
    }

    /// Encodes a map value, from a `MapValue`.
    pub fn encode_map_value(&mut self, value: &MapValue) -> Result<()> {
        self.encode_map(&value.0)
    }

    // MARK: - Header

    /// Encodes a map value's header.
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
                len.with_packed_be_bytes(self.config.lengths.packing, |bytes| {
                    let width = bytes.len() as u8;

                    byte |= (width - 1) & MapHeader::EXTENDED_LEN_WIDTH_BITS;

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

    /// Creates a header for a map value, from its length.
    pub fn header_for_map_len(&self, len: usize) -> MapHeader {
        MapHeader::for_len(len, self.config.lengths.packing)
    }
}
