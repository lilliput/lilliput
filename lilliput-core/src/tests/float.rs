use crate::value::FloatValue;

use super::*;

fn values() -> BoxedStrategy<FloatValue> {
    FloatValue::arbitrary().boxed()
}

proptest! {
    #[test]
    fn roundtrip(value in values()) {
        let profile = Profile::None;

        let mut encoder = Encoder::new(profile);
        encoder.encode_float_value(&value).unwrap();
        let encoded = encoder.into_vec().unwrap();

        let mut decoder = Decoder::new(&encoded, profile);
        let decoded = decoder.decode_float_value().unwrap();
        prop_assert_eq!(&decoded, &value);

        let mut decoder = Decoder::new(&encoded, profile);
        let decoded = decoder.decode_any().unwrap();
        let Value::Float(decoded) = decoded else {
            panic!("expected float value");
        };
        prop_assert_eq!(&decoded, &value);
    }
}
