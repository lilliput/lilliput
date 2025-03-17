use crate::{
    header::MapHeader,
    value::{Map, MapValue, Value},
};

use super::{BufRead, Decoder, DecoderError};

#[derive(Debug)]
pub struct MapDecoder<'de, R> {
    inner: &'de mut Decoder<R>,
}

impl<'de, R> MapDecoder<'de, R>
where
    R: BufRead,
{
    pub(super) fn with(inner: &'de mut Decoder<R>) -> Self {
        Self { inner }
    }

    pub(super) fn decode_map(&mut self) -> Result<Map, DecoderError> {
        let len = self.decode_map_start()?;

        #[cfg(feature = "preserve_order")]
        pub(crate) type Map = ordermap::OrderMap<Value, Value>;

        #[cfg(not(feature = "preserve_order"))]
        pub(crate) type Map = std::collections::BTreeMap<Value, Value>;

        let mut map = Map::default();

        for _ in 0..len {
            let key = self.inner.decode_any()?;
            let value = self.inner.decode_any()?;
            map.insert(key, value);
        }

        let () = self.decode_map_end()?;

        Ok(map)
    }

    pub(super) fn decode_map_value(&mut self) -> Result<MapValue, DecoderError> {
        self.decode_map().map(From::from)
    }

    pub(super) fn decode_map_start(&mut self) -> Result<usize, DecoderError> {
        let header: MapHeader = self.inner.pull_header()?;

        let len = match header {
            MapHeader::Compact { len } => len,
            MapHeader::Extended { len_width } => self.inner.pull_len_bytes(len_width)?,
        };

        Ok(len)
    }

    pub(super) fn decode_map_end(&mut self) -> Result<(), DecoderError> {
        Ok(())
    }
}
