use crate::{
    io::{StdIoBufReader, StdIoWriter},
    value::IntValue,
};

use super::*;

fn values() -> BoxedStrategy<IntValue> {
    IntValue::arbitrary().boxed()
}

proptest! {
    #[test]
    fn roundtrip(value in values()) {
        let profile = Profile::None;

        let writer: StdIoWriter<Vec<u8>> = StdIoWriter(vec![]);
        let mut encoder = Encoder::new(writer, profile);
        encoder.encode_int_value(&value).unwrap();
        let encoded = encoder.into_writer().unwrap().0;

        let reader: StdIoBufReader<&[u8]> = StdIoBufReader(&encoded);
        let mut decoder = Decoder::new(reader, profile);
        let decoded = decoder.decode_int_value().unwrap();
        prop_assert_eq!(&decoded, &value);

        let reader: StdIoBufReader<&[u8]> = StdIoBufReader(&encoded);
        let mut decoder = Decoder::new(reader, profile);
        let decoded = decoder.decode_any().unwrap();
        let Value::Int(decoded) = decoded else {
            panic!("expected int value");
        };
        prop_assert_eq!(&decoded, &value);
    }
}
