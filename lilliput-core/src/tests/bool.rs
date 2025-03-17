use crate::{
    io::{StdIoBufReader, StdIoWriter},
    value::BoolValue,
};

use super::*;

fn values() -> BoxedStrategy<BoolValue> {
    BoolValue::arbitrary().boxed()
}

proptest! {
    #[test]
    fn roundtrip(value in values()) {
        let profile = Profile::None;

        let writer: StdIoWriter<Vec<u8>> = StdIoWriter(vec![]);
        let mut encoder = Encoder::new(writer, profile);
        encoder.encode_bool(value.0).unwrap();
        let encoded = encoder.into_writer().unwrap().0;
        prop_assert_eq!(encoded.len(), 1);

        let reader: StdIoBufReader<&[u8]> = StdIoBufReader(&encoded);
        let mut decoder = Decoder::new(reader, profile);
        let decoded = decoder.decode_bool().unwrap();
        prop_assert_eq!(decoded, value.0);

        let reader: StdIoBufReader<&[u8]> = StdIoBufReader(&encoded);
        let mut decoder = Decoder::new(reader, profile);
        let decoded = decoder.decode_any().unwrap();
        let Value::Bool(decoded) = decoded else {
            panic!("expected bool value");
        };
        prop_assert_eq!(&decoded, &value);
    }
}
