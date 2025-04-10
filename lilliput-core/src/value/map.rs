#[cfg(any(test, feature = "testing"))]
use proptest::{prelude::*, sample::SizeRange};
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

use super::Value;

#[cfg(feature = "preserve_order")]
pub type Map = ordermap::OrderMap<Value, Value>;

#[cfg(not(feature = "preserve_order"))]
pub type Map = std::collections::BTreeMap<Value, Value>;

#[cfg(any(test, feature = "testing"))]
pub(crate) fn arbitrary_map() -> impl Strategy<Value = Map> {
    arbitrary_map_with(Value::arbitrary(), Value::arbitrary(), 0..10)
}

#[cfg(any(test, feature = "testing"))]
pub(crate) fn arbitrary_map_with(
    key: impl Strategy<Value = Value>,
    value: impl Strategy<Value = Value>,
    size: impl Into<SizeRange>,
) -> impl Strategy<Value = Map> {
    proptest::collection::hash_map(key, value, size.into()).prop_map(Map::from_iter)
}

/// Represents a map of key-value pairs.
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MapValue(
    #[cfg_attr(any(test, feature = "testing"), proptest(strategy = "arbitrary_map()"))] pub Map,
);

impl From<Map> for MapValue {
    fn from(value: Map) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a MapValue> for &'a Map {
    fn from(value: &'a MapValue) -> Self {
        &value.0
    }
}

impl From<MapValue> for Map {
    fn from(value: MapValue) -> Self {
        value.0
    }
}

impl std::fmt::Debug for MapValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.0.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use test_log::test;

    use crate::{
        config::EncodingConfig,
        decoder::Decoder,
        encoder::Encoder,
        io::{SliceReader, VecWriter},
        value::{NullValue, Value},
    };

    use super::*;

    #[test]
    fn debug() {
        let mut map = Map::default();
        map.insert(Value::Null(NullValue), Value::Null(NullValue));
        let value = MapValue::from(map);

        assert_eq!(format!("{:?}", value), "{null: null}");
        assert_eq!(
            format!("{:#?}", value),
            "{\n    Null(\n        null,\n    ): Null(\n        null,\n    ),\n}"
        );
    }

    proptest! {
        #[test]
        fn encode_decode_roundtrip(value in MapValue::arbitrary(), config in EncodingConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new(writer, config);
            encoder.encode_map(&value.0).unwrap();

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_map().unwrap();
            prop_assert_eq!(&decoded, &value.0);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::new(reader);
            let decoded = decoder.decode_value().unwrap();
            let Value::Map(decoded) = decoded else {
                panic!("expected map value");
            };
            prop_assert_eq!(&decoded, &value);
        }
    }
}
