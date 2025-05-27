use lilliput_serde::prelude::*;

fn main() {
    // Or any other `T: Serialize`:
    let value = Value::String(StringValue::from("hello world".to_owned()));

    let encoded = to_vec(&value).unwrap();

    // or in case you need more fine-tuning:
    // let config: SerializerConfig = SerializerConfig::default();
    // let encoded = to_vec_with_config(&value, config).unwrap();

    let decoded = from_slice(&encoded).unwrap();

    assert_eq!(value, decoded);
}
