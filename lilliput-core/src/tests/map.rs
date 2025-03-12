use crate::value::MapValue;

use super::*;

fn values() -> BoxedStrategy<MapValue> {
    MapValue::arbitrary().boxed()
}

proptest! {
    #[test]
    fn roundtrip(value in values()) {
        let profile = Profile::None;

        let mut encoder = Encoder::new(profile);
        encoder.encode_map(&value.0).unwrap();
        let encoded = encoder.into_vec().unwrap();

        let mut decoder = Decoder::new(&encoded, profile);
        let decoded = decoder.decode_map().unwrap();
        prop_assert_eq!(&decoded, &value.0);

        let mut decoder = Decoder::new(&encoded, profile);
        let decoded = decoder.decode_any().unwrap();
        let Value::Map(decoded) = decoded else {
            panic!("expected map value");
        };
        prop_assert_eq!(&decoded, &value);
    }
}
