use crate::value::SeqValue;

use super::*;

fn values() -> BoxedStrategy<SeqValue> {
    SeqValue::arbitrary().boxed()
}

proptest! {
    #[test]
    fn roundtrip(value in values()) {
        let profile = Profile::None;

        let mut encoder = Encoder::new(profile);
        encoder.encode_seq(&value.0).unwrap();
        let encoded = encoder.into_vec().unwrap();

        let mut decoder = Decoder::new(&encoded, profile);
        let decoded = decoder.decode_seq().unwrap();
        prop_assert_eq!(&decoded, &value.0);

        let mut decoder = Decoder::new(&encoded, profile);
        let decoded = decoder.decode_any().unwrap();
        let Value::Seq(decoded) = decoded else {
            panic!("expected seq value");
        };
        prop_assert_eq!(&decoded, &value);
    }
}
