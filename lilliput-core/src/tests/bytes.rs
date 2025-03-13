use crate::value::BytesValue;

use super::*;

fn values() -> BoxedStrategy<BytesValue> {
    BytesValue::arbitrary().boxed()
}

proptest! {
    #[test]
    fn roundtrip(value in values()) {
        let profile = Profile::None;

        let mut encoder = Encoder::new(profile);
        encoder.encode_bytes(value.as_slice()).unwrap();
        let encoded = encoder.into_vec().unwrap();

        let mut decoder = Decoder::new(&encoded, profile);
        let decoded = decoder.decode_bytes().unwrap();
        prop_assert_eq!(&decoded, value.as_slice());

        let mut decoder = Decoder::new(&encoded, profile);
        let decoded = decoder.decode_any().unwrap();
        let Value::Bytes(decoded) = decoded else {
            panic!("expected bytes value");
        };
        prop_assert_eq!(&decoded, &value);
    }
}
