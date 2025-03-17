use crate::{
    io::{StdIoBufReader, StdIoWriter},
    value::SeqValue,
};

use super::*;

fn values() -> BoxedStrategy<SeqValue> {
    SeqValue::arbitrary().boxed()
}

proptest! {
    #[test]
    fn roundtrip(value in values()) {
        let profile = Profile::None;

        let writer: StdIoWriter<Vec<u8>> = StdIoWriter(vec![]);
        let mut encoder = Encoder::new(writer, profile);
        encoder.encode_seq(&value.0).unwrap();
        let encoded = encoder.into_writer().unwrap().0;

        let reader: StdIoBufReader<&[u8]> = StdIoBufReader(&encoded);
        let mut decoder = Decoder::new(reader, profile);
        let decoded = decoder.decode_seq().unwrap();
        prop_assert_eq!(&decoded, &value.0);

        let reader: StdIoBufReader<&[u8]> = StdIoBufReader(&encoded);
        let mut decoder = Decoder::new(reader, profile);
        let decoded = decoder.decode_any().unwrap();
        let Value::Seq(decoded) = decoded else {
            panic!("expected seq value");
        };
        prop_assert_eq!(&decoded, &value);
    }
}
