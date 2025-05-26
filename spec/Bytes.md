# Bytes

Represents a sequence of lilliput-encoded values.

## Binary representation

```plain
0b000001XX <INTEGER> <BYTE>*
  ├────┘├┘ ├───────┘ ├─────┘
  │     │  │         └─ Bytes
  │     │  └─ Number of bytes
  │     └─ Length width exponent
  └─ Bytes type
```

where

- `XXX` is a 3-bit unsigned integer which represents the network-endian, bit-packed number of bytes required to represent the value, subtracted by `1`.
- `<INTEGER>` is the exponent of a power-of-two representation (`width = 2 ^ exponent`) of the byte array's length (i.e. number of bytes).
- `<ENCODED>*` is a variable-length sequence of lilliput-encoded values, representing the items of the sequence value.
