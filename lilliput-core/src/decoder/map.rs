use crate::{
    error::Result,
    header::MapHeader,
    marker::Marker,
    value::{Map, MapValue},
};

use super::{Decoder, Read};

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    // MARK: - Value

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_map(&mut self) -> Result<Map> {
        let header = self.decode_map_header()?;
        self.decode_map_of(header)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_map_value(&mut self) -> Result<MapValue> {
        self.decode_map().map(From::from)
    }

    // MARK: - Header

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_map_header(&mut self) -> Result<MapHeader> {
        let byte = self.pull_byte_expecting(Marker::Map)?;

        let is_compact = (byte & MapHeader::COMPACT_VARIANT_BIT) != 0b0;

        if is_compact {
            let len = byte & MapHeader::COMPACT_LEN_BITS;

            #[cfg(feature = "tracing")]
            tracing::debug!(
                byte = crate::binary::fmt_byte(byte),
                is_compact = true,
                len = len
            );

            Ok(MapHeader::compact(len))
        } else {
            let len_width = 1 + (byte & MapHeader::EXTENDED_LEN_WIDTH_BITS);
            let len = self.pull_len_bytes(len_width)?;

            #[cfg(feature = "tracing")]
            tracing::debug!(
                byte = crate::binary::fmt_byte(byte),
                is_compact = false,
                len = len
            );

            Ok(MapHeader::extended(len))
        }
    }

    // MARK: - Body

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decode_map_value_of(&mut self, header: MapHeader) -> Result<MapValue> {
        self.decode_map_of(header).map(From::from)
    }

    // MARK: - Private

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    fn decode_map_of(&mut self, header: MapHeader) -> Result<Map> {
        let mut map = Map::default();

        for _ in 0..header.len() {
            let key = self.decode_value()?;
            let value = self.decode_value()?;
            map.insert(key, value);
        }

        Ok(map)
    }
}
