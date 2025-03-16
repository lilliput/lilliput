#[cfg(test)]
use proptest::{prelude::*, sample::SizeRange};

use super::Value;

#[cfg(feature = "preserve_order")]
pub type Map = ordermap::OrderMap<Value, Value>;

#[cfg(not(feature = "preserve_order"))]
pub type Map = std::collections::BTreeMap<Value, Value>;

/// Represents a map of key-value pairs.
///
/// # Binary representation
///
/// ```plain
/// 0b0001XXXX <INTEGER>? [KEY:VALUE,*]
///   ├──┘│├─┘ ├───────┘  ├───────────┘
///   │   ││   └─ Length? └─ Key-value pairs
///   │   │└─ <depends on variant>
///   │   └─ Variant
///   └─ Map type
/// ```
///
/// ## Compact variant
///
/// ```plain
/// 0b00011XXX [KEY:VALUE,*]
///   ├──┘│├─┘ ├───────────┘
///   │   ││   └─ Key-value pairs
///   │   │└─ Length
///   │   └─ Compact variant
///   └─ Map type
/// ```
///
/// ## Extended variant
///
/// ```plain
/// 0b00010XXX <INTEGER> [KEY:VALUE,*]
///   ├──┘│├─┘ ├───────┘ ├───────────┘
///   │   ││   └─ Length └─ Key-value pairs
///   │   │└─ Number of bytes in length
///   │   └─ Extended variant
///   └─ Map type
/// ```
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MapValue(pub Map);

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

#[doc(hidden)]
#[cfg(test)]
pub struct MapValueArbitraryParameters {
    pub keys: BoxedStrategy<Value>,
    pub values: BoxedStrategy<Value>,
    pub size: SizeRange,
}

#[cfg(test)]
impl Default for MapValueArbitraryParameters {
    fn default() -> Self {
        Self {
            keys: Value::arbitrary(),
            values: Value::arbitrary(),
            size: (0..10).into(),
        }
    }
}

impl std::fmt::Debug for MapValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.0.iter()).finish()
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for MapValue {
    type Parameters = MapValueArbitraryParameters;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        let MapValueArbitraryParameters { keys, values, size } = args;

        proptest::collection::hash_map(keys, values, size)
            .prop_map(|hash_map| MapValue(Map::from_iter(hash_map)))
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use crate::value::NullValue;

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
}
