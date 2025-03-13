use crate::value::IntValue;

use super::*;

fn values() -> BoxedStrategy<IntValue> {
    IntValue::arbitrary().boxed()
}

proptest! {
    #[test]
    fn roundtrip(value in values()) {
        let profile = Profile::None;

        let mut encoder = Encoder::new(profile);
        encoder.encode_int_value(&value).unwrap();
        let encoded = encoder.into_vec().unwrap();

        let mut decoder = Decoder::new(&encoded, profile);
        let decoded = decoder.decode_int_value().unwrap();
        prop_assert_eq!(&decoded, &value);

        let mut decoder = Decoder::new(&encoded, profile);
        let decoded = decoder.decode_any().unwrap();
        let Value::Int(decoded) = decoded else {
            panic!("expected int value");
        };
        prop_assert_eq!(&decoded, &value);
    }
}
