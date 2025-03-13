use crate::value::NullValue;

use super::*;

fn values() -> BoxedStrategy<NullValue> {
    NullValue::arbitrary().boxed()
}

proptest! {
    #[test]
    fn roundtrip(value in values()) {
        let profile = Profile::None;

        let mut encoder = Encoder::new(profile);
        encoder.encode_null().unwrap();
        let encoded = encoder.into_vec().unwrap();

        let mut decoder = Decoder::new(&encoded, profile);
        decoder.decode_null().unwrap();

        let mut decoder = Decoder::new(&encoded, profile);
        let decoded = decoder.decode_any().unwrap();
        let Value::Null(decoded) = decoded else {
            panic!("expected null value");
        };
        prop_assert_eq!(&decoded, &value);
    }
}
