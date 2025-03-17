use crate::{
    io::{StdIoBufReader, StdIoWriter},
    value::NullValue,
};

use super::*;

fn values() -> BoxedStrategy<NullValue> {
    NullValue::arbitrary().boxed()
}

proptest! {
    #[test]
    fn roundtrip(value in values()) {
        let profile = Profile::None;

        let writer: StdIoWriter<Vec<u8>> = StdIoWriter(vec![]);
        let mut encoder = Encoder::new(writer, profile);
        encoder.encode_null().unwrap();
        let encoded = encoder.into_writer().unwrap().0;

        let reader: StdIoBufReader<&[u8]> = StdIoBufReader(&encoded);
        let mut decoder = Decoder::new(reader, profile);
        decoder.decode_null().unwrap();

        let reader: StdIoBufReader<&[u8]> = StdIoBufReader(&encoded);
        let mut decoder = Decoder::new(reader, profile);
        let decoded = decoder.decode_any().unwrap();
        let Value::Null(decoded) = decoded else {
            panic!("expected null value");
        };
        prop_assert_eq!(&decoded, &value);
    }
}
