use crate::{
    io::{StdIoBufReader, StdIoWriter},
    value::BytesValue,
};

use super::*;

fn values() -> BoxedStrategy<BytesValue> {
    BytesValue::arbitrary().boxed()
}

proptest! {
    #[test]
    fn roundtrip(value in values()) {
        let profile = Profile::None;

        let writer: StdIoWriter<Vec<u8>> = StdIoWriter(vec![]);
        let mut encoder = Encoder::new(writer, profile);
        encoder.encode_bytes(value.as_slice()).unwrap();
        let encoded = encoder.into_writer().unwrap().0;

        let reader: StdIoBufReader<&[u8]> = StdIoBufReader(&encoded);
        let mut decoder = Decoder::new(reader, profile);
        let decoded = decoder.decode_bytes().unwrap();
        prop_assert_eq!(&decoded, value.as_slice());

        let reader: StdIoBufReader<&[u8]> = StdIoBufReader(&encoded);
        let mut decoder = Decoder::new(reader, profile);
        let decoded = decoder.decode_any().unwrap();
        let Value::Bytes(decoded) = decoded else {
            panic!("expected bytes value");
        };
        prop_assert_eq!(&decoded, &value);
    }
}
