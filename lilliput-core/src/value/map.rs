#[cfg(any(test, feature = "testing"))]
use proptest::{prelude::*, sample::SizeRange};
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;

use super::Value;

/// An ordered map.
#[cfg(feature = "preserve_order")]
pub type Map = ordermap::OrderMap<Value, Value>;

/// An unordered map.
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

impl MapValue {
    /// Returns a reference to the internal map.
    pub fn as_map_ref(&self) -> &Map {
        &self.0
    }

    /// Returns the internal map, consuming `self`.
    pub fn into_map(self) -> Map {
        self.0
    }

    /// Returns the length of the internal map.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true`, if the internal map is empty, otherwise `false`.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

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

#[cfg(feature = "serde")]
impl serde::Serialize for MapValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MapValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(Map::deserialize(deserializer)?))
    }
}

#[cfg(feature = "serde")]
pub(crate) struct MapKeyClassifier;

#[cfg(feature = "serde")]
pub(crate) enum MapKeyClass {
    Map(Value),
}

#[cfg(feature = "serde")]
impl<'de> serde::de::DeserializeSeed<'de> for MapKeyClassifier {
    type Value = MapKeyClass;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize as _;

        Ok(MapKeyClass::Map(Value::deserialize(deserializer)?))
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use test_log::test;

    use crate::{
        config::EncoderConfig,
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

        assert_eq!(format!("{value:?}"), "{null: null}");
        assert_eq!(
            format!("{value:#?}"),
            "{\n    Null(\n        null,\n    ): Null(\n        null,\n    ),\n}"
        );
    }

    proptest! {
        #[test]
        fn encode_decode_roundtrip(value in MapValue::arbitrary(), config in EncoderConfig::arbitrary()) {
            let mut encoded: Vec<u8> = Vec::new();
            let writer = VecWriter::new(&mut encoded);
            let mut encoder = Encoder::new_with_config(writer, config);
            encoder.encode_map(&value.0).unwrap();

            // the encoded length of a map depends on the items
            // contained within, so we're not checking it here.

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::from_reader(reader);
            let decoded = decoder.decode_map().unwrap();
            prop_assert_eq!(&decoded, &value.0);

            let reader = SliceReader::new(&encoded);
            let mut decoder = Decoder::from_reader(reader);
            let decoded = decoder.decode_value().unwrap();
            let Value::Map(decoded) = decoded else {
                panic!("expected map value");
            };
            prop_assert_eq!(&decoded, &value);
        }
    }
}
