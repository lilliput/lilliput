# lilliput-core

[![Downloads](https://img.shields.io/crates/d/lilliput-core.svg?style=flat-square)](https://crates.io/crates/lilliput-core/)
[![Version](https://img.shields.io/crates/v/lilliput-core.svg?style=flat-square)](https://crates.io/crates/lilliput-core/)
[![License](https://img.shields.io/crates/l/lilliput-core.svg?style=flat-square)](https://crates.io/crates/lilliput-core/)

## Synopsis

Low-level implementation of encoding/decoding logic for lilliput format.

## Usage

```rust
use lilliput_core::prelude::*;

fn main() {
    let value = Value::String(StringValue::from("hello world".to_owned()));

    let mut encoded = Vec::with_capacity(1024);

    let writer = VecWriter::new(&mut encoded);

    let mut encoder = Encoder::new(writer);

    // or in case you need more fine-tuning:
    // let config = EncoderConfig::default();
    // let mut encoder = Encoder::new_with_config(writer, config);

    encoder.encode_value(&value).unwrap();

    let reader = SliceReader::new(&encoded);
    let mut decoder = Decoder::new(reader);

    let decoded = decoder.decode_value().unwrap();

    assert_eq!(value, decoded);
}

```
