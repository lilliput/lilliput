use crate::{
    error::Result,
    header::MapHeader,
    value::{Map, MapValue, Value},
};

use super::{Decoder, Read};

impl<'de, R> Decoder<R>
where
    R: Read<'de>,
{
    pub fn decode_map(&mut self) -> Result<Map> {
        let header: MapHeader = self.pull_header()?;
        self.decode_map_headed_by(header)
    }

    pub fn decode_map_value(&mut self) -> Result<MapValue> {
        let header: MapHeader = self.pull_header()?;
        self.decode_map_value_headed_by(header)
    }

    pub fn decode_map_start(&mut self) -> Result<usize> {
        let header: MapHeader = self.pull_header()?;
        self.decode_map_start_headed_by(header)
    }

    fn decode_map_headed_by(&mut self, header: MapHeader) -> Result<Map> {
        let len = self.decode_map_start_headed_by(header)?;

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

        let () = self.decode_map_end()?;

        Ok(map)
    }

    pub(super) fn decode_map_value_headed_by(&mut self, header: MapHeader) -> Result<MapValue> {
        self.decode_map_headed_by(header).map(From::from)
    }

    fn decode_map_start_headed_by(&mut self, header: MapHeader) -> Result<usize> {
        let len = match header {
            MapHeader::Compact { len } => len,
            MapHeader::Extended { len_width } => self.pull_len_bytes(len_width)?,
        };

        Ok(len)
    }

    pub fn decode_map_end(&mut self) -> Result<()> {
        Ok(())
    }
}
