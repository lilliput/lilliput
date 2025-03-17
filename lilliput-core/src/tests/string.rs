use crate::{
    io::{StdIoBufReader, StdIoWriter},
    value::StringValue,
};

use super::*;

fn values() -> BoxedStrategy<StringValue> {
    StringValue::arbitrary().boxed()
}

proptest! {
    #[test]
    fn roundtrip(value in values()) {
        let profile = Profile::None;

        let writer: StdIoWriter<Vec<u8>> = StdIoWriter(vec![]);
        let mut encoder = Encoder::new(writer, profile);
        encoder.encode_string(&value.0).unwrap();
        let encoded = encoder.into_writer().unwrap().0;

        let reader: StdIoBufReader<&[u8]> = StdIoBufReader(&encoded);
        let mut decoder = Decoder::new(reader, profile);
        let decoded = decoder.decode_string().unwrap();
        prop_assert_eq!(&decoded, &value.0);

        let reader: StdIoBufReader<&[u8]> = StdIoBufReader(&encoded);
        let mut decoder = Decoder::new(reader, profile);
        let decoded = decoder.decode_any().unwrap();
        let Value::String(decoded) = decoded else {
            panic!("expected string value");
        };
        prop_assert_eq!(&decoded, &value);
    }
}
