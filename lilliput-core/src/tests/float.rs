use crate::{
    io::{StdIoBufReader, StdIoWriter},
    value::FloatValue,
};

use super::*;

fn values() -> BoxedStrategy<FloatValue> {
    FloatValue::arbitrary().boxed()
}

proptest! {
    #[test]
    fn roundtrip(value in values()) {
        let profile = Profile::None;

        let writer: StdIoWriter<Vec<u8>> = StdIoWriter(vec![]);
        let mut encoder = Encoder::new(writer, profile);
        encoder.encode_float_value(&value).unwrap();
        let encoded = encoder.into_writer().unwrap().0;

        let reader: StdIoBufReader<&[u8]> = StdIoBufReader(&encoded);
        let mut decoder = Decoder::new(reader, profile);
        let decoded = decoder.decode_float_value().unwrap();
        prop_assert_eq!(&decoded, &value);

        let reader: StdIoBufReader<&[u8]> = StdIoBufReader(&encoded);
        let mut decoder = Decoder::new(reader, profile);
        let decoded = decoder.decode_any().unwrap();
        let Value::Float(decoded) = decoded else {
            panic!("expected float value");
        };
        prop_assert_eq!(&decoded, &value);
    }
}
