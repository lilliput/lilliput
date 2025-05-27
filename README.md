# lilliput

Lilliput is an object serialization specification like JSON, but binary, with a focus on compactness.

While JSON aims to provide a short, yet human readable object representation, Lilliput gives up readability for the sake of the following features:

1. Compact binary representation
2. Fast streaming serialization/deserialization
3. Low-memory streaming usage
4. Zero-copy/alloc wherever possible
5. Good inline compression
6. Type-safety

## [lilliput-serde](./lilliput-serde)

A serializer and deserializer of the lilliput data format, for serde.

## [lilliput-core](./lilliput-core)

Low-level implementation of encoding/decoding logic for lilliput format.

## [lilliput-float](./lilliput-float)

IEEE-754-compliant float-packing implementation, used in lilliput-core.
