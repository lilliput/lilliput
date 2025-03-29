use crate::{
    error::Result,
    header::{MapHeader, MapHeaderRepr},
    value::{Map, MapValue, Value},
};

use super::{Decoder, Read};

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    pub fn decode_map(&mut self) -> Result<Map> {
        let len = self.decode_map_header()?;

        #[cfg(feature = "preserve_order")]
        pub(crate) type Map = ordermap::OrderMap<Value, Value>;

        #[cfg(not(feature = "preserve_order"))]
        pub(crate) type Map = std::collections::BTreeMap<Value, Value>;

        let mut map = Map::default();

        for _ in 0..len {
            let key = self.decode_any()?;
            let value = self.decode_any()?;
            map.insert(key, value);
        }

        Ok(map)
    }

    pub fn decode_map_value(&mut self) -> Result<MapValue> {
        self.decode_map().map(From::from)
    }

    pub fn decode_map_header(&mut self) -> Result<usize> {
        let header: MapHeader = self.pull_header()?;

        let len: usize = match header.repr() {
            MapHeaderRepr::Compact { len } => len.into(),
            MapHeaderRepr::Extended { len_width } => self.pull_len_bytes(len_width)?,
        };

        Ok(len)
    }
}
