# lilliput-serde

[![Downloads](https://img.shields.io/crates/d/lilliput-serde.svg?style=flat-square)](https://crates.io/crates/lilliput-serde/)
[![Version](https://img.shields.io/crates/v/lilliput-serde.svg?style=flat-square)](https://crates.io/crates/lilliput-serde/)
[![License](https://img.shields.io/crates/l/lilliput-serde.svg?style=flat-square)](https://crates.io/crates/lilliput-serde/)

## Synopsis

A serializer and deserializer of the lilliput data format, for serde.

## Usage

```rust
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
```
