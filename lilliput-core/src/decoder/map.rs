use crate::value::{Map, MapValue, Value, ValueType};

use super::{Decoder, DecoderError};

#[derive(Debug)]
pub struct MapDecoder<'a, 'de> {
    inner: &'de mut Decoder<'a>,
}

impl<'a, 'de> MapDecoder<'a, 'de> {
    pub(super) fn with(inner: &'de mut Decoder<'a>) -> Self {
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
        let byte = self.inner.pull_byte_expecting_type(ValueType::Map)?;

        let is_long = byte & MapValue::VARIANT_BIT != 0b0;

        if is_long {
            let len_width_exponent = (byte & MapValue::LONG_LEN_WIDTH_BITS) as u32;
            let len_width = 1_usize << len_width_exponent;

            let mut bytes: [u8; 8] = [0b0; 8];

            let range = {
                let start = 8 - len_width;
                let end = start + len_width;
                start..end
            };

            bytes[range].copy_from_slice(self.inner.pull_bytes(len_width)?);

            let len = u64::from_be_bytes(bytes) as usize;

            if self.inner.remaining_len() < len {
                return Err(DecoderError::Eof);
            }

            Ok(len)
        } else {
            Err(DecoderError::IncompatibleProfile)
        }
    }

    pub(super) fn decode_map_end(&mut self) -> Result<(), DecoderError> {
        Ok(())
    }
}
