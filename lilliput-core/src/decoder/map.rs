use crate::{
    error::Result,
    header::MapHeader,
    marker::Marker,
    value::{Map, MapValue, Value},
};

use super::{Decoder, Read};

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    pub fn decode_map(&mut self) -> Result<Map> {
        let header = self.decode_map_header()?;

        #[cfg(feature = "preserve_order")]
        pub(crate) type Map = ordermap::OrderMap<Value, Value>;

        #[cfg(not(feature = "preserve_order"))]
        pub(crate) type Map = std::collections::BTreeMap<Value, Value>;

        let mut map = Map::default();

        for _ in 0..header.len() {
            let key = self.decode_value()?;
            let value = self.decode_value()?;
            map.insert(key, value);
        }

        Ok(map)
    }

    pub fn decode_map_value(&mut self) -> Result<MapValue> {
        self.decode_map().map(From::from)
    }

    pub fn decode_map_header(&mut self) -> Result<MapHeader> {
        let header_byte = self.pull_byte_expecting(Marker::Map)?;

        let is_compact = (header_byte & MapHeader::COMPACT_VARIANT_BIT) != 0b0;

        if is_compact {
            let len = header_byte & MapHeader::COMPACT_LEN_BITS;
            Ok(MapHeader::compact(len))
        } else {
            let len_width = 1 + (header_byte & MapHeader::EXTENDED_LEN_WIDTH_BITS);
            let len = self.pull_len_bytes(len_width)?;
            Ok(MapHeader::extended(len))
        }
    }
}
