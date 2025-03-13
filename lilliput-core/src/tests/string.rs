use crate::value::StringValue;

use super::*;

fn values() -> BoxedStrategy<StringValue> {
    StringValue::arbitrary().boxed()
}

proptest! {
    #[test]
    fn roundtrip(value in values()) {
        let profile = Profile::None;

        let mut encoder = Encoder::new(profile);
        encoder.encode_string(&value.0).unwrap();
        let encoded = encoder.into_vec().unwrap();

        let mut decoder = Decoder::new(&encoded, profile);
        let decoded = decoder.decode_string().unwrap();
        prop_assert_eq!(&decoded, &value.0);

        let mut decoder = Decoder::new(&encoded, profile);
        let decoded = decoder.decode_any().unwrap();
        let Value::String(decoded) = decoded else {
            panic!("expected string value");
        };
        prop_assert_eq!(&decoded, &value);
    }
}
