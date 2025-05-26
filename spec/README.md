# Format Specification

Lilliput is an object serialization specification like JSON, but binary, with a focus on compactness.

While JSON aims to provide a short, yet human readable object representation, Lilliput gives up readability for the sake of the following features:

1. Compact binary representation
2. Fast streaming serialization/deserialization
3. Low-memory streaming usage
4. Zero-copy/alloc wherever possible
5. Good inline compression
6. Type-safety

## Supported Types

Lilliput supports encoding values as the following types:

- Types
  - [**Integer**](./Integer.md) represents a integer number:
    - signed:
      - 8-bit
      - 16-bit
      - 24-bit
      - 32-bit
      - 40-bit
      - 48-bit
      - 56-bit
      - 64-bit
    - unsigned:
      - 8-bit
      - 16-bit
      - 24-bit
      - 32-bit
      - 40-bit
      - 48-bit
      - 56-bit
      - 64-bit
  - [**String**](./String.md) represents a utf8-encoded textual value.
  - [**Sequence**](./Sequence.md) represents a length-prefixed sequence of values.
  - [**Map**](./Map.md) represents a length-prefixed map of key-value pairs.
  - [**Float**](./Float.md) represents a floating-point value:
    - 8-bit
    - 16-bit
    - 24-bit
    - 32-bit
    - 40-bit
    - 48-bit
    - 56-bit
    - 64-bit
  - [**Bytes**](./Bytes.md) represents a length-prefixed byte array.
  - [**Bool**](./Bool.md) represents a boolean value.
  - [**Unit**](./Unit.md) represents a unit value.
  - [**Null**](./Null.md) represents a null value.

## Value Representation

The general representation pattern of lilliput-encoded values is:

```plain
<HEADER> <EXTENSION>? <VALUE>?
├──────┘ ├──────────┘ ├──────┘
│        │            └─ Value (optional)
│        └─ Header extension (optional)
└─ Value header
```

where

- `<HEADER>` is an 8-bit header byte that specifies the value's type, as well as type-specific meta information about the value and/or its representation.
- `<EXTENSION>` is a variable-byte header extension that specifies additional representation size information about the value. For values that fit within the header byte this may be omitted.
- `<VALUE>` is a variable-byte byte sequence that represents the actual value. For values that fit within the header byte this may be omitted.
