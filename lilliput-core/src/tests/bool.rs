use crate::value::BoolValue;

use super::*;

fn values() -> BoxedStrategy<BoolValue> {
    BoolValue::arbitrary().boxed()
}

proptest! {
    #[test]
    fn roundtrip(value in values()) {
        let profile = Profile::None;

        let mut encoder = Encoder::new(profile);
        encoder.encode_bool(value.0).unwrap();
        let encoded = encoder.into_vec().unwrap();
        prop_assert_eq!(encoded.len(), 1);

        let mut decoder = Decoder::new(&encoded, profile);
        let decoded = decoder.decode_bool().unwrap();
        prop_assert_eq!(decoded, value.0);

        let mut decoder = Decoder::new(&encoded, profile);
        let decoded = decoder.decode_any().unwrap();
        let Value::Bool(decoded) = decoded else {
            panic!("expected bool value");
        };
        prop_assert_eq!(&decoded, &value);
    }
}
